use axum::{routing::{get, post}, Router};
use deadpool_diesel::postgres::{Pool};

use crate::controllers;

// build application with routes
pub fn routes(pool: Pool) -> Router {
    let auth_routes = Router::new()
        .route("/register/", post(controllers::auth::register))
        .route("/login/", post(controllers::auth::login))
        .route("/verify_email/", get(controllers::auth::verify_email))
        .route("/logout/", get(controllers::auth::logout))
        .route("/user/", get(controllers::auth::get_user));

    let post_routes = Router::new()
        .route("/", get(controllers::post::get_post_list))
        .route("/", post(controllers::post::create_post))
        .route("/:id/", get(controllers::post::get_post));

    let comment_routes = Router::new()
        .route("/", post(controllers::comment::create_comment))
        .route("/", get(controllers::comment::get_comment_list));

    let admin_user_routes = Router::new()
        .route("/user/", get(controllers::user::get_user_list))
        .route("/post/", get(controllers::post::get_post_list))
        .route("/comment/", get(controllers::comment::get_comment_list));

    let api_routes = Router::new()
        .route("/", get(controllers::home::get_home))
        .nest("/auth/", auth_routes)
        .nest("/post/", post_routes)
        .nest("/comment/", comment_routes)
        .nest("/admin/", admin_user_routes);

    Router::new()
        .nest("/api/", api_routes)
        .with_state(pool)
}
