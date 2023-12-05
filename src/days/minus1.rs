use axum::http::StatusCode;

pub async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fake_error() {
        assert_eq!(fake_error().await, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
