
use rocket::form::FromForm;
use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    user_id: u64,
    username: String,
    #[schemars(example = "example_email")]
    email: Option<String>,
}

pub fn example_email() -> &'static str {
    "test@example.com"
}

/// # Get all users
///
/// Returns all users in the system.
#[openapi(tag = "Users")]
#[get("/user")]
pub fn get_all_users() -> Json<Vec<User>> {
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

/// # Get user
///
/// Returns a single user by ID.
#[openapi(tag = "Users")]
#[get("/user/<id>")]
pub fn get_user(id: u64) -> Option<Json<User>> {
    Some(Json(User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }))
}

/// # Get user by name
///
/// Returns a single user by username.
#[openapi(tag = "Users")]
#[get("/user_example?<user_id>&<name>&<email>")]
pub fn get_user_by_name(user_id: u64, name: String, email: Option<String>) -> Option<Json<User>> {
    Some(Json(User {
        user_id,
        username: name,
        email,
    }))
}

/// # Create user
#[openapi(tag = "Users")]
#[post("/user", data = "<user>")]
pub fn create_user(user: Json<User>) -> Json<User> {
    user
}

#[openapi(skip)]
#[get("/hidden")]
pub fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}

#[derive(Serialize, Deserialize, JsonSchema, FromForm)]
pub struct Post {
    /// The unique identifier for the post.
    post_id: u64,
    /// The title of the post.
    title: String,
    /// A short summary of the post.
    summary: Option<String>,
}

/// # Create post using query params
///
/// Returns the created post.
#[openapi(tag = "Posts")]
#[get("/post_by_query?<post..>")]
pub fn create_post_by_query(post: Post) -> Option<Json<Post>> {
    Some(Json(post))
}