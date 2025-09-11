use async_smtp::{Envelope, SendableEmail, SmtpClient, SmtpTransport};
use chrono::Duration;
use tokio::{io::BufStream, net::TcpStream};

use crate::{
    types::{email::Email, phone::PhoneNumber},
    utils::error::{ApiError, ApiResult},
};

pub async fn send_verification_email(to: &Email, otp: &str, ttl: Duration) -> ApiResult<()> {
    let subject = "Subject: Verify your email\n";
    let from = "From: no-reply@localhost\n";
    let to_header = format!("To: {}\n", to.as_str());
    let content_type = "Content-Type: text/plain; charset=utf-8\n";
    let blank_line = "\n";

    let msg_body = format!(
        "Your email verification OTP is: {}\nPlease don't share it with anyone.",
        otp
    );

    let msg = format!(
        "{}{}{}{}{}{}",
        from, to_header, subject, content_type, blank_line, msg_body
    );

    let mail_server =
        std::env::var("MAIL_SERVER").map_err(|_| ApiError::internal("Can't get mail server"))?;
    let stream = TcpStream::connect(&mail_server)
        .await
        .map_err(|_| ApiError::internal("Can't connect to SMTP server"))?;

    let reader = BufStream::new(stream);
    let client = SmtpClient::new();

    let mut transport = SmtpTransport::new(client, reader)
        .await
        .map_err(|_| ApiError::internal("Can't create the transport layer"))?;

    let envelope = Envelope::new(
        Some("no-reply@localhost".parse().unwrap()),
        vec![to.as_str().parse().unwrap()],
    )
    .map_err(|_| ApiError::internal("Can't create the email envelope"))?;

    let email = SendableEmail::new(envelope, msg);

    transport
        .send(email)
        .await
        .map_err(|_| ApiError::internal("Can't send email"))?;

    Ok(())
}

pub async fn send_reset_email(to: &Email, otp: &str, ttl: Duration) -> ApiResult<()> {
    let subject = "Subject: Reset your password\n";
    let from = "From: no-reply@localhost\n";
    let to_header = format!("To: {}\n", to.as_str());
    let content_type = "Content-Type: text/plain; charset=utf-8\n";
    let blank_line = "\n";

    let msg_body = format!(
        "Your password reset OTP is: {}\nPlease don't share it with anyone.",
        otp
    );

    let msg = format!(
        "{}{}{}{}{}{}",
        from, to_header, subject, content_type, blank_line, msg_body
    );

    let mail_server =
        std::env::var("MAIL_SERVER").map_err(|_| ApiError::internal("Can't get mail server"))?;
    let stream = TcpStream::connect(&mail_server)
        .await
        .map_err(|_| ApiError::internal("Can't connect to SMTP server"))?;

    let reader = BufStream::new(stream);
    let client = SmtpClient::new();

    let mut transport = SmtpTransport::new(client, reader)
        .await
        .map_err(|_| ApiError::internal("Can't create the transport layer"))?;

    let envelope = Envelope::new(
        Some("no-reply@localhost".parse().unwrap()),
        vec![to.as_str().parse().unwrap()],
    )
    .map_err(|_| ApiError::internal("Can't create the email envelope"))?;

    let email = SendableEmail::new(envelope, msg);

    transport
        .send(email)
        .await
        .map_err(|_| ApiError::internal("Can't send email"))?;

    Ok(())
}

pub async fn send_verification_otp(to: &PhoneNumber, otp: &str) -> ApiResult<()> {
    let subject = "Subject: Verify your phone\n";
    let from = "From: no-reply@localhost\n";
    let to_header = format!("To: {}@sms.localhost\n", to.as_str());
    let content_type = "Content-Type: text/plain; charset=utf-8\n";
    let blank_line = "\n";

    let msg_body = format!(
        "Your phone verification OTP is: {}\nPlease don't share it with anyone.",
        otp
    );

    let msg = format!(
        "{}{}{}{}{}{}",
        from, to_header, subject, content_type, blank_line, msg_body
    );

    let stream = TcpStream::connect("127.0.0.1:1025")
        .await
        .map_err(|_| ApiError::internal("Can't connect to SMTP server"))?;

    let reader = BufStream::new(stream);
    let client = SmtpClient::new();

    let mut transport = SmtpTransport::new(client, reader)
        .await
        .map_err(|_| ApiError::internal("Can't create the transport layer"))?;

    let envelope = Envelope::new(
        Some("no-reply@localhost".parse().unwrap()),
        vec![to.as_str().parse().unwrap()],
    )
    .map_err(|_| ApiError::internal("Can't create the email envelope"))?;

    let email = SendableEmail::new(envelope, msg);

    transport
        .send(email)
        .await
        .map_err(|_| ApiError::internal("Can't send email"))?;

    Ok(())
}
