use actix_web::{get, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    let response = HealthResponse {
        status: "OK".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: HealthResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "OK");
        // Verify timestamp format (basic check)
        assert!(chrono::DateTime::parse_from_rfc3339(&body.timestamp).is_ok());
    }
}
