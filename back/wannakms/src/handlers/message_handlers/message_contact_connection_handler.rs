use crate::schemas::message_schemas::message_contact_connection_schemas;
use crate::AppState;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/my-side/add/")]
async fn add_contact_user_wired(
    pool: web::Data<AppState>,
    new_contact: web::Json<message_contact_connection_schemas::NewWireContact>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        INSERT INTO user_wired_connection_keys (nick, user_secret_key, other_secret_key, is_connection_possible, connection_link)
        VALUES (?, ?, ?, ?, ?)
    ";

    let result = query(query_str)
        .bind(&new_contact.nick)
        .bind(&new_contact.user_secret_key)
        .bind(&new_contact.other_side_secret_key)
        .bind(&new_contact.is_connection_possible)
        .bind(&new_contact.connection_link)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error adding contact: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/other-side/add/")]
async fn add_contact_other_wired(
    pool: web::Data<AppState>,
    new_contact: web::Json<message_contact_connection_schemas::NewWireContact>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        INSERT INTO other_wired_connection_keys (other_nick, user_secret_key, other_secret_key, is_connection_possible, connection_link)
        VALUES (?, ?, ?, ?, ?)
    ";

    let result = query(query_str)
        .bind(&new_contact.nick)
        .bind(&new_contact.user_secret_key)
        .bind(&new_contact.other_side_secret_key)
        .bind(&new_contact.is_connection_possible)
        .bind(&new_contact.connection_link)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error adding contact: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
