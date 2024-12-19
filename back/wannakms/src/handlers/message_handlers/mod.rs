mod message_auth_handlers;
mod message_contact_connection_handler;
mod message_contact_handlers;
mod message_get_set_handlers;
mod message_handler_package;

use message_contact_handlers::{
    get_other_server_people_handler, get_phone_number_other_server, get_phone_number_user_server,
    get_social_media_other_server, get_social_media_user_server, get_user_server_people_handler,
    modify_other_wired_people_extra_info, modify_phone_number_other_wired,
    modify_phone_number_user_server, modify_social_media_user_wired, modify_social_media_wired,
    modify_user_wired_people_extra_info,
};

use message_get_set_handlers::{
    get_messages_other_client, get_messages_user_client, link_file_to_message,
    send_message_other_client, send_message_user_client, upload_file,
};

use message_auth_handlers::{
    check_auth_body, check_auth_identity, check_auth_talk_with_god, check_auth_whole_program,
    check_auth_wires, fetch_from_tor, hello_world,
};

use message_contact_connection_handler::{add_contact_other_wired, add_contact_user_wired};

use message_handler_package::{
    reset_connected_people_table_handler, reset_connecting_people_table_handler,
    reset_messages_send_to_other_client_table_handler,
    reset_messages_send_to_user_client_table_handler,
};

pub fn message_handler_config(conf: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("/api/message")
        .service(reset_messages_send_to_user_client_table_handler)
        .service(reset_messages_send_to_other_client_table_handler)
        .service(send_message_user_client)
        .service(send_message_other_client)
        .service(get_messages_user_client)
        .service(get_messages_other_client)
        .service(link_file_to_message)
        .service(upload_file)
        .service(reset_connecting_people_table_handler)
        .service(reset_connected_people_table_handler)
        .service(get_user_server_people_handler)
        .service(modify_other_wired_people_extra_info)
        .service(modify_phone_number_other_wired)
        .service(get_other_server_people_handler)
        .service(modify_phone_number_user_server)
        .service(modify_social_media_user_wired)
        .service(modify_social_media_wired)
        .service(modify_user_wired_people_extra_info)
        .service(get_phone_number_user_server)
        .service(get_phone_number_other_server)
        .service(get_social_media_user_server)
        .service(get_social_media_other_server)
        .service(add_contact_other_wired)
        .service(add_contact_user_wired)
        .service(check_auth_body)
        .service(check_auth_identity)
        .service(check_auth_talk_with_god)
        .service(check_auth_whole_program)
        .service(check_auth_wires)
        .service(fetch_from_tor)
        .service(hello_world);
    conf.service(scope);
}
