use check_if_email_exists::{check_email, CheckEmailInput, Reachable};

pub async fn check(email: &str) -> bool {
    let input = CheckEmailInput::new(email.to_string());
    let result = check_email(&input).await;
    result.is_reachable != Reachable::Invalid
}
