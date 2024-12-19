use crate::schemas::message_schemas::message_auth_schemas;
// use crate::AppState;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use dotenv::dotenv;
use reqwest;
use sqlx::query_as;
use std::env;

// The handler to check credentials from POST request
#[post("/auth/check/body")]
pub async fn check_auth_body(
    user_entering_data: web::Json<message_auth_schemas::Credentials>,
) -> impl Responder {
    // Load environment variables from .env
    dotenv().ok();

    // Get the username and password from .env
    let env_username = env::var("USERNAME_BODY")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_this_username".to_string());
    let env_password = env::var("PASSWORD_BODY")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_this_password".to_string());

    // Compare the incoming credentials with the ones in .env
    if user_entering_data.username == Some(env_username)
        && user_entering_data.password == Some(env_password)
    {
        // Create an authenticated user response
        let user = message_auth_schemas::AuthenticatedUser {
            username: user_entering_data
                .username
                .clone()
                .unwrap_or_else(|| "you are an idiot".to_string()),
        };
        HttpResponse::Ok().json(user) // Respond with the JSON of the authenticated user
    } else {
        HttpResponse::Unauthorized()
            .body("Bro, stop wasting your time with this crap and go get a boring desk job")
    }
}

// The handler to check credentials from POST request
#[post("/auth/check/wires")]
pub async fn check_auth_wires(
    user_entering_data: web::Json<message_auth_schemas::Credentials>,
) -> impl Responder {
    // Load environment variables from .env
    dotenv().ok();

    // Get the username and password from .env
    let env_username = env::var("USERNAME_WIRES")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_this_username".to_string());
    let env_password = env::var("PASSWORD_WIRES")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_this_password".to_string());

    // Compare the incoming credentials with the ones in .env
    if user_entering_data.username == Some(env_username)
        && user_entering_data.password == Some(env_password)
    {
        // Create an authenticated user response
        let user = message_auth_schemas::AuthenticatedUser {
            username: user_entering_data
                .username
                .clone()
                .unwrap_or_else(|| "you are an idiot".to_string()),
        };
        HttpResponse::Ok().json(user) // Respond with the JSON of the authenticated user
    } else {
        HttpResponse::Unauthorized()
            .body("Bro, stop wasting your time with this crap and go get a boring desk job")
    }
}

// The handler to check credentials from POST request
#[post("/auth/check/identity")]
pub async fn check_auth_identity(
    user_entering_data: web::Json<message_auth_schemas::Credentials>,
) -> impl Responder {
    // Load environment variables from .env
    dotenv().ok();

    // Get the username and password from .env
    let env_username = env::var("USERNAME_IDENTITY")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_this_username".to_string());
    let env_password = env::var("PASSWORD_IDENTITY")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_this_password".to_string());

    // Compare the incoming credentials with the ones in .env
    if user_entering_data.username == Some(env_username)
        && user_entering_data.password == Some(env_password)
    {
        // Create an authenticated user response
        let user = message_auth_schemas::AuthenticatedUser {
            username: user_entering_data
                .username
                .clone()
                .unwrap_or_else(|| "you are an idiot".to_string()),
        };
        HttpResponse::Ok().json(user) // Respond with the JSON of the authenticated user
    } else {
        HttpResponse::Unauthorized()
            .body("Bro, stop wasting your time with this crap and go get a boring desk job")
    }
}

// The handler to check credentials from POST request
#[post("/auth/check/talk_with_god")]
pub async fn check_auth_talk_with_god(
    user_entering_data: web::Json<message_auth_schemas::Credentials>,
) -> impl Responder {
    // Load environment variables from .env
    dotenv().ok();

    // Get the username and password from .env
    let env_username = env::var("USERNAME_TALK_WITH_GOD")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_this_username".to_string());
    let env_password = env::var("PASSWORD_TALK_WITH_GOD")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_this_password".to_string());

    // Compare the incoming credentials with the ones in .env
    if user_entering_data.username == Some(env_username)
        && user_entering_data.password == Some(env_password)
    {
        // Create an authenticated user response
        let user = message_auth_schemas::AuthenticatedUser {
            username: user_entering_data
                .username
                .clone()
                .unwrap_or_else(|| "you are an idiot".to_string()),
        };

        HttpResponse::Ok().json(user) // Respond with the JSON of the authenticated user
    } else {
        HttpResponse::Unauthorized()
            .body("Bro, stop wasting your time with this crap and go get a boring desk job")
    }
}

// The handler to check credentials from POST request
#[post("/auth/check/whole_program")]
pub async fn check_auth_whole_program(
    user_entering_data: web::Json<message_auth_schemas::EnteringApplicationData>,
) -> impl Responder {
    // Load environment variables from .env
    dotenv().ok();

    // Get the application username from the environment variables
    let env_application_username = env::var("APPLICATION_USERNAME")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_APPLICATION_USERNAME".to_string());

    // Get the username and password from .env
    let env_application_password = env::var("APPLICATION_PASSWORD")
        .unwrap_or_else(|_| "You_idiot_you_forgot_adding_APPLICATION_PASSWORD".to_string());

    // Compare the incoming credentials with the ones in .env
    if user_entering_data.application_password == Some(env_application_password)
        && user_entering_data.application_username == Some(env_application_username)
    {
        // Create an authenticated user response
        let user = message_auth_schemas::AuthenticatedUser {
            username: user_entering_data
                .application_username
                .clone()
                .unwrap_or_else(|| "you are an idiot".to_string()),
        };

        HttpResponse::Ok().json(user) // Respond with the JSON of the authenticated user
    } else {
        HttpResponse::Unauthorized()
            .body("Bro, stop wasting your time with this crap and go get a boring desk job")
    }
}

// Handler function for "Hello, World!"
#[get("/hello-world-to-ferfer")]
pub async fn hello_world() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("Hello, World ferfer!"))
}

#[post("/auth/check/wired_connection")]
async fn check_auth_wired_connection_secure(
    pool: web::Data<AppState>,
    contact_want_connected: web::Json<message_auth_schemas::IsAccessibleCheckingData>,
) -> impl Responder {
    dotenv::dotenv().ok(); // Load environment variables from .env file

    // Retrieve the user link from the environment variable
    let env_user_link = env::var("MY_ONION_LINK").unwrap_or_else(|_| {
        eprintln!("Missing USER_LINK_INSIDE_ENV_FILE in the .env file.");
        "Invalid_User_Link".to_string() // Fallback in case the environment variable is missing
    });

    // SQL query to check if the record exists with all necessary conditions
    let query_str = "
        SELECT * FROM other_wired_people
        WHERE user_nick = ? 
          AND user_secret_key = ? 
          AND other_secret_key = ? 
          AND connection_link = ? 
          AND is_connection_possible = true
    ";

    // Check if the user can connect based on `IsAccessibleCheckingData`
    match query_as::<_, message_auth_schemas::IsAccessibleCheckingData>(query_str)
        .bind(&contact_want_connected.connecting_user_name)
        .bind(&contact_want_connected.user_password)
        .bind(&contact_want_connected.other_password)
        .bind(&contact_want_connected.link_the_user_want_connect)
        .fetch_optional(&pool.db)
        .await
    {
        Ok(Some(_)) => {
            // User found, now check if the connection URL is accessible
            let connection_check_url = &contact_want_connected.link_the_user_want_connect;

            // First, check if the connection URL is reachable via a GET request
            let client = reqwest::Client::new();
            let get_response = client.get(connection_check_url).send().await;

            match get_response {
                Ok(resp) if resp.status().is_success() => {
                    // Connection is successful, proceed to check user existence
                    let user_exists_url =
                        format!("{}/api/auth/check/user_exists", connection_check_url);

                    // Create a modified instance with env_user_link
                    let modified_contact = message_auth_schemas::IsAccessibleCheckingData {
                        connecting_user_name: contact_want_connected.user_password.clone(),
                        user_password: contact_want_connected.connecting_user_name.clone(),
                        other_password: contact_want_connected.other_password.clone(),
                        link_the_user_want_connect: env_user_link.clone(), // Replace with env_user_link
                    };

                    // Send user data via POST request to check if the user exists with a timeout
                    let post_response = client
                        .post(user_exists_url)
                        .json(&modified_contact) // Send modified user data
                        .timeout(std::time::Duration::from_secs(15)) // Set a timeout of 15 seconds
                        .send()
                        .await;

                    match post_response {
                        Ok(post_resp) => {
                            if post_resp.status().is_success() {
                                // Connection confirmed
                                return HttpResponse::Ok()
                                    .json("Connection secured and user exists.");
                            } else {
                                // Connection rejected
                                return HttpResponse::Forbidden()
                                    .body("Connection rejected by the other side.");
                            }
                        }
                        Err(e) => {
                            if e.is_timeout() {
                                return HttpResponse::RequestTimeout()
                                    .body("Connection timed out while checking user existence.");
                            } else {
                                eprintln!("Error sending POST request: {}", e);
                                return HttpResponse::InternalServerError()
                                    .body("Failed to send POST request.");
                            }
                        }
                    }
                }
                Ok(_) => {
                    // Connection failed
                    return HttpResponse::ServiceUnavailable()
                        .body("The service at the specified link is unavailable.");
                }
                Err(e) => {
                    eprintln!("Error checking connection: {}", e);
                    return HttpResponse::InternalServerError()
                        .body("Failed to check the connection.");
                }
            }
        }
        Ok(None) => HttpResponse::Unauthorized()
            .body("Bro, stop wasting your time with this crap and go get a boring desk job"),
        Err(e) => {
            eprintln!("Error retrieving connecting people: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/auth/check/user_exists")]
async fn check_user_exists(
    pool: web::Data<AppState>,
    user_data: web::Json<message_auth_schemas::IsAccessibleCheckingData>,
) -> impl Responder {
    // SQL query to check if the user exists with the necessary conditions
    let query_str = "
        SELECT * FROM other_wired_people
        WHERE user_nick = ? 
          AND user_secret_key = ? 
          AND other_secret_key = ?
          AND connection_link = ? 
          AND is_connection_possible = true
    ";

    // Check if the user exists
    match query_as::<_, message_auth_schemas::IsAccessibleCheckingData>(query_str)
        .bind(&user_data.connecting_user_name)
        .bind(&user_data.user_password)
        .bind(&user_data.other_password)
        .bind(&user_data.link_the_user_want_connect) // Use the connection link from user data
        .fetch_optional(&pool.db)
        .await
    {
        Ok(Some(_)) => HttpResponse::Ok().json("User exists and can connect."),
        Ok(None) => HttpResponse::NotFound().body("User not found or cannot connect."),
        Err(e) => {
            eprintln!("Error checking if user exists: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/api/wanted")]
async fn fetch_from_tor() -> impl Responder {
    // Proxy yapılandırması (socks5 proxy kullanılıyor)
    let proxy = match reqwest::Proxy::all("socks5://127.0.0.1:9051") {
        Ok(proxy) => proxy,
        Err(e) => {
            println!("Proxy yapılandırma hatası: {:?}", e);
            return HttpResponse::InternalServerError().body("Proxy yapılandırılamadı.");
        }
    };

    // HTTP istemcisi oluşturma ve zaman aşımı ayarı
    let client = match reqwest::Client::builder()
        .proxy(proxy)
        .timeout(std::time::Duration::from_secs(20)) // 20 saniye zaman aşımı
        .build()
    {
        Ok(client) => client,
        Err(e) => {
            println!("İstemci oluşturma hatası: {:?}", e);
            return HttpResponse::InternalServerError().body("İstemci oluşturulamadı.");
        }
    };

    // .onion URL'si
    let url = "http://xxprmdidsnetjutwsmbs23clklfnmpi5l4gtezq557czq5xafxe4lxid.onion/api/message/hello-world-to-ferfer";

    // GET isteği gönderme
    match client.get(url).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(_) => HttpResponse::InternalServerError().body("Cevap metni okunamadı."),
        },
        Err(e) => {
            println!("GET isteği hatası: {:?}", e);
            HttpResponse::InternalServerError().body("Tor ağına bağlanırken hata oluştu.")
        }
    }
}
