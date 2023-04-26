use bcrypt::{hash, verify, DEFAULT_COST};
use check_if_email_exists::{check_email as check, CheckEmailInput, Reachable};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn hash_password(password: &String) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &String, hashed: &String) -> bool {
    verify(password, hashed).unwrap()
}

pub async fn check_email(email: &str) -> bool {
    let input = CheckEmailInput::new(email.to_string());
    let result = check(&input).await;
    result.is_reachable != Reachable::Invalid
}

pub fn send_email(email: &str, title: &str, content: &str) -> bool {
    let mut email_str = "User <".to_string();
    email_str.push_str(email);
    email_str.push_str(">");

    let email = Message::builder()
        .from("zwd.xyz <cyxhuan9@163.com>".parse().unwrap())
        .to(email_str.parse().unwrap())
        .subject(title)
        .header(ContentType::TEXT_PLAIN)
        .body(content.to_string())
        .unwrap();

    let creds = Credentials::new("cyxhuan9".to_owned(), "qqshSJHc*yyjcc".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.163.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email).is_ok()
}
