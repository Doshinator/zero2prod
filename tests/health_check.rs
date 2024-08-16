//! tests/health_check.rs
//! no need to add $[cfg(test)],
//! Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test

use zero2prod::health_check;

mod tests {
    use super::*;
    use actix_web::{test, App};

    #[rustfmt::skip]
    #[actix_web::test] 
    async fn health_check_success() {
        // Arrange
        let app = test::init_service(
            App::new()
                .service(health_check)
        )
        .await;

        // Act
        let req = test::TestRequest::get().uri("/health_check").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }
}
