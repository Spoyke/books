use axum::{Router, routing::get};

async fn index() -> &'static str {
    "Message received"
}

async fn health() -> &'static str {
    "ok"
}

fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server is listening on 127.0.0.1:3000");

    axum::serve(listener, app).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, Bytes};
    use axum::http::{Request, StatusCode, header};
    use axum::response::Response;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn send_request(method: &str, path: &str) -> Response {
        let request = Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap();

        app().oneshot(request).await.unwrap()
    }

    async fn read_body(response: Response) -> Bytes {
        response.into_body().collect().await.unwrap().to_bytes()
    }

    #[tokio::test]
    async fn health_returns_ok() {
        let response = send_request("GET", "/health").await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = read_body(response).await;
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn index_returns_ok() {
        let response = send_request("GET", "/").await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = read_body(response).await;
        assert_eq!(&body[..], b"Message received");
    }

    #[tokio::test]
    async fn unknown_path_returns_404() {
        let response = send_request("GET", "/nonsense").await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn unsupported_method_returns_405() {
        let response = send_request("POST", "/").await;

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(
            response
                .headers()
                .get(header::ALLOW)
                .expect("Missing 'allow' header"),
            "GET,HEAD"
        );
    }
}
