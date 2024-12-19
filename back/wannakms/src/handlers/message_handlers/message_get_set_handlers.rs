use crate::schemas::message_schemas::message_get_set_schemas;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/user/get/{connected}")]
pub async fn get_messages_user_client(
    pool: web::Data<AppState>,
    connected: web::Path<String>,
) -> impl Responder {
    let query_str = "
        SELECT id, sender, receiver, content, timestamp, close_one_point, connected, has_attachment 
        FROM messages_send_to_user_client 
        WHERE connected = ? 
        ORDER BY timestamp DESC
    ";

    match sqlx::query_as::<_, message_get_set_schemas::Message>(query_str)
        .bind(connected.as_str())
        .fetch_all(&pool.db)
        .await
    {
        Ok(messages) => {
            let response: Vec<message_get_set_schemas::MessageResponse> =
                messages.into_iter().map(|m| m.to_response()).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Error retrieving messages from 'user-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/other/get/{connected}")]
pub async fn get_messages_other_client(
    pool: web::Data<AppState>,
    connected: web::Path<String>,
) -> impl Responder {
    let query_str = "
        SELECT id, sender, receiver, content, timestamp, close_one_point, connected, has_attachment 
        FROM messages_send_to_other_client 
        WHERE connected = ? 
        ORDER BY timestamp DESC
    ";

    match sqlx::query_as::<_, message_get_set_schemas::Message>(query_str)
        .bind(connected.as_str())
        .fetch_all(&pool.db)
        .await
    {
        Ok(messages) => {
            let response: Vec<message_get_set_schemas::MessageResponse> =
                messages.into_iter().map(|m| m.to_response()).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Error retrieving messages from 'other-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/user/send/")]
pub async fn send_message_user_client(
    pool: web::Data<AppState>,
    new_message: web::Json<message_get_set_schemas::NewMessage>,
) -> impl Responder {
    let query_str = "
        INSERT INTO messages_send_to_user_client (sender, receiver, content, close_one_point, connected, has_attachment)
        VALUES (?, ?, ?, ?, ?, ?)
    ";

    let result = sqlx::query(query_str)
        .bind(&new_message.sender)
        .bind(&new_message.receiver)
        .bind(&new_message.content)
        .bind(&new_message.close_one_point)
        .bind(&new_message.connected)
        .bind(&new_message.has_attachment)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error inserting message into 'user-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/other/send/")]
pub async fn send_message_other_client(
    pool: web::Data<AppState>,
    new_message: web::Json<message_get_set_schemas::NewMessage>,
) -> impl Responder {
    let query_str = "
        INSERT INTO messages_send_to_other_client (sender, receiver, content, close_one_point, connected, has_attachment)
        VALUES (?, ?, ?, ?, ?, ?)
    ";

    let result = sqlx::query(query_str)
        .bind(&new_message.sender)
        .bind(&new_message.receiver)
        .bind(&new_message.content)
        .bind(&new_message.close_one_point)
        .bind(&new_message.connected)
        .bind(&new_message.has_attachment)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error inserting message into 'other-client' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/upload/file")]
pub async fn upload_file(
    pool: web::Data<AppState>,
    file_info: web::Json<message_get_set_schemas::NewFile>,
) -> impl Responder {
    let query_str = "
        INSERT INTO sent_files (file_name, file_type, file_size, file_data)
        VALUES (?, ?, ?, ?)
    ";

    let result = sqlx::query(query_str)
        .bind(&file_info.file_name)
        .bind(&file_info.file_type)
        .bind(&file_info.file_size)
        .bind(&file_info.file_data)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error inserting file into 'sent_files' table: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/link/file")]
pub async fn link_file_to_message(
    pool: web::Data<AppState>,
    file_link: web::Json<(i32, i32, String)>, // message_id, file_id, target (user or other)
) -> impl Responder {
    let (message_id, file_id, target) = file_link.into_inner();

    let query_str = match target.as_str() {
        "user" => {
            "INSERT INTO messages_send_to_user_client_files (message_id, file_id) VALUES (?, ?)"
        }
        "other" => {
            "INSERT INTO messages_send_to_other_client_files (message_id, file_id) VALUES (?, ?)"
        }
        _ => return HttpResponse::BadRequest().finish(),
    };

    let result = sqlx::query(query_str)
        .bind(message_id)
        .bind(file_id)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error linking file to message: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
