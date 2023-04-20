use axum::{routing::{get, post}, Router};
use deadpool_diesel::postgres::{Pool};

use crate::controllers;

// build application with routes
pub fn routes(pool: Pool) -> Router {
    let auth_routes = Router::new()
        .route("/register/", post(controllers::auth::register))
        .route("/login/", post(controllers::auth::login))
        .route("/logout/", get(controllers::auth::logout))
        .route("/user/", get(controllers::auth::get_user));

    let post_routes = Router::new()
        .route("/", get(controllers::post::get_post_list))
        .route("/", post(controllers::post::create_post))
        .route("/:id/", get(controllers::post::get_post));

    let api_routes = Router::new()
        // home
        .route("/", get(controllers::home::get_home))
        // auth
        .nest("/auth/", auth_routes)
        // TODO: user
        // TODO: comment
        // post
        .nest("/post/", post_routes);

    Router::new()
        .nest("/api/", api_routes)
        .with_state(pool)
}
