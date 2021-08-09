#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

mod api_v1;
mod rebrickable;

use rocket_okapi::swagger_ui::*;

fn test() -> () {
    let db = rebrickable::load_db();

    println!("loaded db");
}

#[rocket::main]
async fn main() {
    test();
    let result = rocket::build()
        .mount(
            "/",
            routes_with_openapi![
                api_v1::create_post_by_query,
                api_v1::get_all_users,
                api_v1::get_user,
                api_v1::get_user_by_name,
                api_v1::create_user,
                api_v1::hidden,
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                deep_linking: true,
                display_request_duration: true,
                url: "../openapi.json".to_owned(),
                ..SwaggerUIConfig::default()
            }),
        )
        .launch()
        .await;
    assert!(result.is_ok());
}
