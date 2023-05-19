use axum::{routing::{get, post}, Router};
use deadpool_diesel::postgres::Pool;

use crate::modules::*;

// build application with routes
pub fn routes(pool: Pool) -> Router {
    let auth_routes = Router::new()
        .route("/register/", post(auth::controllers::register))
        .route("/login/", post(auth::controllers::login))
        .route("/verify_email/", get(auth::controllers::verify_email))
        .route("/logout/", get(auth::controllers::logout))
        .route("/user/", get(auth::controllers::get_user));

    let post_routes = Router::new()
        .route("/", get(post::controllers::get_post_list))
        .route("/author/:id/", get(post::controllers::get_post_list_by_author_id))
        .route("/collected/", get(post::controllers::get_post_list_by_collected))
        .route("/", post(post::controllers::create_post))
        .route("/:id/", get(post::controllers::get_post));

    let comment_routes = Router::new()
        .route("/", post(comment::controllers::create_comment))
        .route("/", get(comment::controllers::get_comment_list));

    let collected_post_routes = Router::new()
        .route("/", post(collected_post::controllers::create_collected_post))
        .route("/", get(collected_post::controllers::get_collected_post_list));

    let admin_user_routes = Router::new()
        .route("/user/", get(user::controllers::get_user_list))
        .route("/post/", get(post::controllers::get_post_list))
        .route("/comment/", get(comment::controllers::get_comment_list))
        .route("/collected_post/", get(collected_post::controllers::get_collected_post_list));

    let api_routes = Router::new()
        .route("/", get(home::controllers::get_home))
        .nest("/auth/", auth_routes)
        .nest("/post/", post_routes)
        .nest("/comment/", comment_routes)
        .nest("/collected_post/", collected_post_routes)
        .nest("/admin/", admin_user_routes);

    Router::new()
        .nest("/api/", api_routes)
        .with_state(pool)
}
