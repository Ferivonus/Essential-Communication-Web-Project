use crate::schemas::form_schemas::form_get_set_schemas;
use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder, Result};

// Define the handler function
#[get("/all-form-pages")]
pub async fn get_all_form_pages(db_pool: web::Data<AppState>) -> Result<impl Responder> {
    let form_pages_result = sqlx::query_as::<_, form_get_set_schemas::FormPage>(
        "SELECT id, form_name, created_at, updated_at, is_active FROM form_pages",
    )
    .fetch_all(&db_pool.db)
    .await;

    match form_pages_result {
        Ok(form_pages) => Ok(HttpResponse::Ok().json(form_pages)),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err); // Log the error
            Ok(HttpResponse::InternalServerError().finish()) // Respond with a 500 error
        }
    }
}

// Handler function for "Hello, World!"
#[get("/hello-world-to-orkun")]
pub async fn hello_world() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("Hello, World ORKUN!"))
}
