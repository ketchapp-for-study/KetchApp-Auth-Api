use actix_web::{post, test, web, App, HttpResponse, Responder};
use actix_web::web::Json;
use authentication::models::register::RegisterUser;

// 1. Define the trait
pub trait UserRepository: Send + Sync {
    fn create_user(&self, user: &RegisterUser) -> Result<(), String>;
}

// 2. Implement for real DB (not shown) and for mock
pub struct MockUserRepo;
impl UserRepository for MockUserRepo {
    fn create_user(&self, _user: &RegisterUser) -> Result<(), String> {
        Ok(()) // or Err("error".into()) for failure cases
    }
}

// 3. Update handler to use trait object
#[post("/register")]
async fn register_handler(
    repo: web::Data<Box<dyn UserRepository>>,
    body: Json<RegisterUser>,
) -> impl Responder {
    let user = body.into_inner();
    match repo.create_user(&user) {
        Ok(_) => HttpResponse::Ok().body(format!(
            "Registration successful for user: ${}",
            user.username
        )),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// 4. In test, inject the mock
#[actix_web::test]
async fn test_register_handler_success() {
    let mock_repo = Box::new(MockUserRepo);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_repo as Box<dyn UserRepository>))
            .service(register_handler),
    )
    .await;

    let payload = serde_json::json!({
        "username": "johndoe",
        "email": "john_doe@gmail.com",
        "password": "Secret123!"
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}
