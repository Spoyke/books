use axum::extract::State;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;

async fn index() -> &'static str {
    "Message received"
}

async fn health(State(pool): State<SqlitePool>) -> Result<&'static str, StatusCode> {
    sqlx::query("SELECT 1").execute(&pool).await.map_err(|e| {
        eprintln!("{}", e);
        StatusCode::SERVICE_UNAVAILABLE
    })?;
    Ok("ok")
}

fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .with_state(pool)
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
        let body = read_body(response).await;
        assert_eq!(&body[..], b"Message received");
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
}
