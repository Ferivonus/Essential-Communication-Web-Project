use crate::schemas::message_schemas;
use crate::AppState;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use sqlx::query_as;

#[put("/user/modify/people/")]
async fn modify_other_wired_people_extra_info(
    pool: web::Data<AppState>,
    updated_contact: web::Json<message_schemas::message_contact_schemas::UpdateUserExtraInfo>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        UPDATE user_wired_people_extra_info
        SET user_nick = ?, name = ?, surname = ?, age = ?, location = ?, occupation = ?, extra_info = ?
        WHERE user_nick = ?
    ";

    let result = query(query_str)
        .bind(&updated_contact.new_nick)
        .bind(&updated_contact.name)
        .bind(&updated_contact.surname)
        .bind(&updated_contact.age)
        .bind(&updated_contact.location)
        .bind(&updated_contact.occupation)
        .bind(&updated_contact.extra_info)
        .bind(&updated_contact.user_nick)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error updating contact: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/other/modify/people/")]
async fn modify_user_wired_people_extra_info(
    pool: web::Data<AppState>,
    updated_contact: web::Json<message_schemas::message_contact_schemas::UpdateUserExtraInfo>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        UPDATE other_wired_people_extra_info
        SET other_nick = ?, name = ?, surname = ?, age = ?, location = ?, occupation = ?, extra_info = ?
        WHERE other_nick = ?
    ";

    let result = query(query_str)
        .bind(&updated_contact.new_nick)
        .bind(&updated_contact.name)
        .bind(&updated_contact.surname)
        .bind(&updated_contact.age)
        .bind(&updated_contact.location)
        .bind(&updated_contact.occupation)
        .bind(&updated_contact.extra_info)
        .bind(&updated_contact.user_nick)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error updating contact: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/user/modify/phone/")]
async fn modify_phone_number_user_server(
    pool: web::Data<AppState>,
    phone_data: web::Json<message_schemas::message_contact_schemas::UpdatePhoneNumber>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        UPDATE user_wired_phone_numbers
        SET phone_number = ?
        WHERE user_nick = ?
    ";

    let result = query(query_str)
        .bind(&phone_data.phone_number)
        .bind(&phone_data.nick)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error modifying phone number: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/other/modify/phone/")]
async fn modify_phone_number_other_wired(
    pool: web::Data<AppState>,
    phone_data: web::Json<message_schemas::message_contact_schemas::UpdatePhoneNumber>,
) -> impl Responder {
    use sqlx::query;
    let query_str = "
        UPDATE other_wired_phone_numbers
        SET phone_number = ?
        WHERE other_nick = ?
    ";

    let result = query(query_str)
        .bind(&phone_data.phone_number)
        .bind(&phone_data.nick)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error modifying phone number: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/user/modify/socials/")]
async fn modify_social_media_user_wired(
    pool: web::Data<AppState>,
    new_social_data: web::Json<message_schemas::message_contact_schemas::UpdateSocialMedia>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        UPDATE user_wired_social_media
        SET phone_number = ?, facebook = ?, instagram = ?, github = ?, website = ?, extra_social = ?
        WHERE user_nick = ?
    ";

    let result = query(query_str)
        .bind(&new_social_data.facebook)
        .bind(&new_social_data.instagram)
        .bind(&new_social_data.github)
        .bind(&new_social_data.website)
        .bind(&new_social_data.extra_social)
        .bind(&new_social_data.nick)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error modifying social media info: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/other/modify/socials/")]
async fn modify_social_media_wired(
    pool: web::Data<AppState>,
    new_social_data: web::Json<message_schemas::message_contact_schemas::UpdateSocialMedia>,
) -> impl Responder {
    use sqlx::query;

    let query_str = "
        UPDATE other_wired_social_media
        SET phone_number = ?, facebook = ?, instagram = ?, github = ?, website = ?, extra_social = ?
        WHERE other_nick = ?
    ";

    let result = query(query_str)
        .bind(&new_social_data.facebook)
        .bind(&new_social_data.instagram)
        .bind(&new_social_data.github)
        .bind(&new_social_data.website)
        .bind(&new_social_data.extra_social)
        .bind(&new_social_data.nick)
        .execute(&pool.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error modifying social media info: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/user/get/people")]
pub async fn get_user_server_people_handler(pool: web::Data<AppState>) -> impl Responder {
    let query_str = "SELECT * FROM user_wired_people";
    match query_as::<_, message_schemas::message_contact_schemas::ProcessedPerson>(query_str)
        .fetch_all(&pool.db)
        .await
    {
        Ok(people) => HttpResponse::Ok().json(people),
        Err(e) => {
            eprintln!("Error retrieving connected people: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/other/get/people")]
pub async fn get_other_server_people_handler(pool: web::Data<AppState>) -> impl Responder {
    let query_str = "SELECT * FROM other_wired_people";
    match query_as::<_, message_schemas::message_contact_schemas::ProcessedPerson>(query_str)
        .fetch_all(&pool.db)
        .await
    {
        Ok(people) => HttpResponse::Ok().json(people),
        Err(e) => {
            eprintln!("Error retrieving connecting people: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// New API points to retrieve phone numbers

#[get("/user/get/phone/{user_nick}")]
pub async fn get_phone_number_user_server(
    pool: web::Data<AppState>,
    user_nick: web::Path<String>,
) -> impl Responder {
    let query_str = "SELECT * FROM user_wired_phone_numbers WHERE user_nick = ?";
    match query_as::<_, message_schemas::message_contact_schemas::ProcessedPhoneNumber>(query_str)
        .bind(&user_nick.into_inner())
        .fetch_one(&pool.db)
        .await
    {
        Ok(phone) => HttpResponse::Ok().json(phone),
        Err(e) => {
            eprintln!("Error retrieving phone number: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/other/get/phone/{other_nick}")]
pub async fn get_phone_number_other_server(
    pool: web::Data<AppState>,
    other_nick: web::Path<String>,
) -> impl Responder {
    let query_str = "SELECT * FROM other_wired_phone_numbers WHERE other_nick = ?";
    match query_as::<_, message_schemas::message_contact_schemas::ProcessedPhoneNumber>(query_str)
        .bind(&other_nick.into_inner())
        .fetch_one(&pool.db)
        .await
    {
        Ok(phone) => HttpResponse::Ok().json(phone),
        Err(e) => {
            eprintln!("Error retrieving phone number: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// New API points to retrieve social media info
#[get("/user/get/socials/{user_nick}")]
pub async fn get_social_media_user_server(
    pool: web::Data<AppState>,
    user_nick: web::Path<String>,
) -> impl Responder {
    let query_str = "SELECT * FROM user_wired_social_media WHERE user_nick = ?";
    match query_as::<_, message_schemas::message_contact_schemas::ProcessedSocialMedia>(query_str)
        .bind(&user_nick.into_inner())
        .fetch_one(&pool.db)
        .await
    {
        Ok(socials) => HttpResponse::Ok().json(socials),
        Err(e) => {
            eprintln!("Error retrieving social media info: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/other/get/socials/{other_nick}")]
pub async fn get_social_media_other_server(
    pool: web::Data<AppState>,
    other_nick: web::Path<String>,
) -> impl Responder {
    let query_str = "SELECT * FROM other_wired_social_media WHERE other_nick = ?";
    match query_as::<_, message_schemas::message_contact_schemas::ProcessedSocialMedia>(query_str)
        .bind(&other_nick.into_inner())
        .fetch_one(&pool.db)
        .await
    {
        Ok(socials) => HttpResponse::Ok().json(socials),
        Err(e) => {
            eprintln!("Error retrieving social media info: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

//get full user info:
