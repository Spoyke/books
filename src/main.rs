use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};
use serde::Serialize;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use tower_http::services::ServeDir;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
enum ReadingStatus {
    Pending,
    Active,
    Completed,
    Dropped,
}

struct LibraryRow {
    uuid: String,
    title: String,
    published_year: Option<i64>,
    status: ReadingStatus,
    authors: String,
}

#[derive(Serialize)]
struct LibraryEntry {
    uuid: String,
    title: String,
    published_year: Option<i64>,
    status: ReadingStatus,
    authors: Vec<String>,
}

async fn health(State(pool): State<SqlitePool>) -> Result<&'static str, StatusCode> {
    sqlx::query("SELECT 1").execute(&pool).await.map_err(|e| {
        eprintln!("{}", e);
        StatusCode::SERVICE_UNAVAILABLE
    })?;
    Ok("ok")
}

async fn library(State(pool): State<SqlitePool>) -> Result<Json<Vec<LibraryEntry>>, StatusCode> {
    let rows = sqlx::query_file_as!(LibraryRow, "queries/library.sql", 1i64)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let entries: Vec<LibraryEntry> = rows
        .into_iter()
        .map(|library_row| {
            let authors: Vec<String> = serde_json::from_str(&library_row.authors)?;
            Ok(LibraryEntry {
                uuid: library_row.uuid,
                title: library_row.title,
                published_year: library_row.published_year,
                status: library_row.status,
                authors,
            })
        })
        .collect::<Result<Vec<_>, serde_json::Error>>()
        .map_err(|e| {
            eprintln!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(entries))
}

fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/library", get(library))
        .with_state(pool)
        .fallback_service(ServeDir::new("frontend"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = SqliteConnectOptions::new()
        .filename("books.db")
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!().run(&pool).await?;
    let app = app(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server is listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, Bytes};
    use axum::http::{Request, header};
    use axum::response::Response;
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use sqlx::sqlite::SqlitePoolOptions;
    use tower::ServiceExt;

    async fn create_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        pool
    }

    async fn send_request(method: &str, path: &str, pool: SqlitePool) -> Response {
        let request = Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap();

        app(pool).oneshot(request).await.unwrap()
    }

    async fn read_body(response: Response) -> Bytes {
        response.into_body().collect().await.unwrap().to_bytes()
    }

    // --- Test data helpers ---

    /// Inserts the single v1 user (id 1) and returns its id.
    async fn insert_user(pool: &SqlitePool) -> i64 {
        sqlx::query_scalar(
            "INSERT INTO users (id, uuid, name) VALUES (1, 'user-1', 'Reader') RETURNING id",
        )
        .fetch_one(pool)
        .await
        .unwrap()
    }

    /// Inserts a book with one edition and its authors, and returns the edition id.
    /// The book's uuid is "book-<title>".
    async fn insert_book(
        pool: &SqlitePool,
        title: &str,
        published_year: Option<i64>,
        authors: &[&str],
    ) -> i64 {
        let book_id: i64 =
            sqlx::query_scalar("INSERT INTO books (uuid, title) VALUES (?, ?) RETURNING id")
                .bind(format!("book-{title}"))
                .bind(title)
                .fetch_one(pool)
                .await
                .unwrap();

        // Authors are inserted in reverse order, so that their insertion order
        // differs from their position: the query must sort them by position.
        for (index, name) in authors.iter().enumerate().rev() {
            let position = index as i64 + 1;
            let author_id: i64 =
                sqlx::query_scalar("INSERT INTO authors (uuid, name) VALUES (?, ?) RETURNING id")
                    .bind(format!("author-{title}-{position}"))
                    .bind(name)
                    .fetch_one(pool)
                    .await
                    .unwrap();

            sqlx::query("INSERT INTO book_authors (book_id, author_id, position) VALUES (?, ?, ?)")
                .bind(book_id)
                .bind(author_id)
                .bind(position)
                .execute(pool)
                .await
                .unwrap();
        }

        sqlx::query_scalar(
            "INSERT INTO editions (uuid, book_id, published_year) VALUES (?, ?, ?) RETURNING id",
        )
        .bind(format!("edition-{title}"))
        .bind(book_id)
        .bind(published_year)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    /// Inserts a reading of an edition, with an explicit creation date
    /// so that the order of the readings is deterministic.
    async fn insert_reading(
        pool: &SqlitePool,
        user_id: i64,
        edition_id: i64,
        status: &str,
        created_at: &str,
    ) {
        sqlx::query(
            "INSERT INTO readings (uuid, user_id, edition_id, status, created_at)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(format!("reading-{edition_id}-{created_at}"))
        .bind(user_id)
        .bind(edition_id)
        .bind(status)
        .bind(created_at)
        .execute(pool)
        .await
        .unwrap();
    }

    /// Calls GET /api/library and returns the parsed JSON body.
    async fn get_library(pool: SqlitePool) -> Value {
        let response = send_request("GET", "/api/library", pool).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = read_body(response).await;
        serde_json::from_slice(&body).expect("Response body is not valid JSON")
    }

    #[tokio::test]
    async fn health_returns_ok() {
        let response = send_request("GET", "/health", create_pool().await).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = read_body(response).await;
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn index_returns_ok() {
        let response = send_request("GET", "/", create_pool().await).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(header::CONTENT_TYPE)
                .expect("Missing 'content-type' header"),
            "text/html"
        );
    }

    #[tokio::test]
    async fn unknown_path_returns_404() {
        let response = send_request("GET", "/nonsense", create_pool().await).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn unsupported_method_returns_405() {
        let response = send_request("POST", "/", create_pool().await).await;

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(
            response
                .headers()
                .get(header::ALLOW)
                .expect("Missing 'allow' header"),
            "GET,HEAD"
        );
    }

    #[tokio::test]
    async fn health_returns_503_when_db_is_unavailable() {
        let pool = create_pool().await;
        pool.close().await;

        let response = send_request("GET", "/health", pool).await;

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    // --- GET /api/library ---

    #[tokio::test]
    async fn library_returns_json() {
        let response = send_request("GET", "/api/library", create_pool().await).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(header::CONTENT_TYPE)
                .expect("Missing 'content-type' header"),
            "application/json"
        );
    }

    #[tokio::test]
    async fn library_is_empty_without_readings() {
        let pool = create_pool().await;
        insert_user(&pool).await;

        assert_eq!(get_library(pool).await, json!([]));
    }

    #[tokio::test]
    async fn library_lists_books_with_their_details() {
        let pool = create_pool().await;
        let user_id = insert_user(&pool).await;
        let dune = insert_book(&pool, "Dune", Some(1965), &["Frank Herbert"]).await;
        insert_reading(&pool, user_id, dune, "completed", "2026-01-01T10:00:00Z").await;

        assert_eq!(
            get_library(pool).await,
            json!([{
                "uuid": "book-Dune",
                "title": "Dune",
                "published_year": 1965,
                "status": "completed",
                "authors": ["Frank Herbert"],
            }])
        );
    }

    #[tokio::test]
    async fn library_shows_null_for_missing_publication_year() {
        let pool = create_pool().await;
        let user_id = insert_user(&pool).await;
        let book = insert_book(&pool, "Unknown year", None, &["Anonymous"]).await;
        insert_reading(&pool, user_id, book, "pending", "2026-01-01T10:00:00Z").await;

        let library = get_library(pool).await;
        assert_eq!(library[0]["published_year"], Value::Null);
    }

    #[tokio::test]
    async fn library_lists_most_recent_books_first() {
        let pool = create_pool().await;
        let user_id = insert_user(&pool).await;
        let dune = insert_book(&pool, "Dune", Some(1965), &["Frank Herbert"]).await;
        let mort = insert_book(&pool, "Mort", Some(1987), &["Terry Pratchett"]).await;
        insert_reading(&pool, user_id, mort, "pending", "2026-03-01T10:00:00Z").await;
        insert_reading(&pool, user_id, dune, "completed", "2026-01-01T10:00:00Z").await;

        let library = get_library(pool).await;
        let titles: Vec<&str> = library
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| entry["title"].as_str().unwrap())
            .collect();
        assert_eq!(titles, ["Mort", "Dune"]);
    }

    #[tokio::test]
    async fn library_keeps_authors_in_order() {
        let pool = create_pool().await;
        let user_id = insert_user(&pool).await;
        let book = insert_book(
            &pool,
            "Good Omens",
            Some(1990),
            &["Terry Pratchett", "Neil Gaiman"],
        )
        .await;
        insert_reading(&pool, user_id, book, "completed", "2026-01-01T10:00:00Z").await;

        let library = get_library(pool).await;
        assert_eq!(
            library[0]["authors"],
            json!(["Terry Pratchett", "Neil Gaiman"])
        );
    }

    #[tokio::test]
    async fn reread_book_appears_once_with_latest_status() {
        let pool = create_pool().await;
        let user_id = insert_user(&pool).await;
        let dune = insert_book(&pool, "Dune", Some(1965), &["Frank Herbert"]).await;
        // The most recent reading is inserted first, so that the insertion order
        // differs from the chronological order: the query must sort by date.
        insert_reading(&pool, user_id, dune, "active", "2026-01-01T10:00:00Z").await;
        insert_reading(&pool, user_id, dune, "completed", "2025-01-01T10:00:00Z").await;

        let library = get_library(pool).await;
        assert_eq!(library.as_array().unwrap().len(), 1);
        assert_eq!(library[0]["status"], "active");
    }
}
