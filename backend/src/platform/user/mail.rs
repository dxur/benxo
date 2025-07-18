use async_smtp::{Envelope, SendableEmail, SmtpClient, SmtpTransport};
use chrono::Duration;
use tokio::{io::BufStream, net::TcpStream};

use crate::{
    platform::user::api::VerificationToken,
    types::{email::Email, phone::PhoneNumber},
    utils::{
        error::{ApiError, ApiResult},
        jwt::encode_jwt,
    },
};

pub async fn send_verification_email(
    to: Email,
    token: VerificationToken,
    ttl: Duration,
) -> ApiResult<()> {
    let token = encode_jwt(token, ttl)?;

    let subject = "Subject: Verify your email\n";
    let from = "From: no-reply@localhost\n";
    let to_header = format!("To: {}\n", to.as_str());
    let content_type = "Content-Type: text/plain; charset=utf-8\n";
    let blank_line = "\n";

    let msg_body = format!(
        "Your verification token is: {}\nPlease don't share it with anyone",
        token
    );

    let msg = format!(
        "{}{}{}{}{}{}",
        from, to_header, subject, content_type, blank_line, msg_body
    );

    let stream =
        TcpStream::connect("127.0.0.1:1025")
            .await
            .map_err(|_| ApiError::InternalError {
                message: "Can't connect to SMTP server".to_string(),
            })?;

    let reader = BufStream::new(stream);
    let client = SmtpClient::new();

    let mut transport =
        SmtpTransport::new(client, reader)
            .await
            .map_err(|_| ApiError::InternalError {
                message: "Can't create the transport layer".to_string(),
            })?;

    let envelope = Envelope::new(
        Some("no-reply@localhost".parse().unwrap()),
        vec![to.as_str().parse().unwrap()],
    )
    .map_err(|_| ApiError::InternalError {
        message: "Can't create the email envelope".to_string(),
    })?;

    let email = SendableEmail::new(envelope, msg);

    transport
        .send(email)
        .await
        .map_err(|_| ApiError::InternalError {
            message: "Can't send email".to_string(),
        })?;

    Ok(())
}

pub async fn send_reset_email(to: Email, token: VerificationToken, ttl: Duration) -> ApiResult<()> {
    let token = encode_jwt(token, ttl)?;

    let subject = "Subject: Reset your password\n";
    let from = "From: no-reply@localhost\n";
    let to_header = format!("To: {}\n", to.as_str());
    let content_type = "Content-Type: text/plain; charset=utf-8\n";
    let blank_line = "\n";

    let msg_body = format!(
        "Your reset token is: {}\nPlease don't share it with anyone",
        token
    );

    let msg = format!(
        "{}{}{}{}{}{}",
        from, to_header, subject, content_type, blank_line, msg_body
    );

    let stream =
        TcpStream::connect("127.0.0.1:1025")
            .await
            .map_err(|_| ApiError::InternalError {
                message: "Can't connect to SMTP server".to_string(),
            })?;

    let reader = BufStream::new(stream);
    let client = SmtpClient::new();

    let mut transport =
        SmtpTransport::new(client, reader)
            .await
            .map_err(|_| ApiError::InternalError {
                message: "Can't create the transport layer".to_string(),
            })?;

    let envelope = Envelope::new(
        Some("no-reply@localhost".parse().unwrap()),
        vec![to.as_str().parse().unwrap()],
    )
    .map_err(|_| ApiError::InternalError {
        message: "Can't create the email envelope".to_string(),
    })?;

    let email = SendableEmail::new(envelope, msg);

    transport
        .send(email)
        .await
        .map_err(|_| ApiError::InternalError {
            message: "Can't send email".to_string(),
        })?;

    Ok(())
}

pub async fn send_verification_otp(to: PhoneNumber, otp: &str) -> ApiResult<()> {
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

    let stream =
        TcpStream::connect("127.0.0.1:1025")
            .await
            .map_err(|_| ApiError::InternalError {
                message: "Can't connect to SMTP server".to_string(),
            })?;

    let reader = BufStream::new(stream);
    let client = SmtpClient::new();

    let mut transport =
        SmtpTransport::new(client, reader)
            .await
            .map_err(|_| ApiError::InternalError {
                message: "Can't create the transport layer".to_string(),
            })?;

    let envelope = Envelope::new(
        Some("no-reply@localhost".parse().unwrap()),
        vec![to.as_str().parse().unwrap()],
    )
    .map_err(|_| ApiError::InternalError {
        message: "Can't create the email envelope".to_string(),
    })?;

    let email = SendableEmail::new(envelope, msg);

    transport
        .send(email)
        .await
        .map_err(|_| ApiError::InternalError {
            message: "Can't send email".to_string(),
        })?;

    Ok(())
}
