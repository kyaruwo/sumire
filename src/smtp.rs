use axum::http::StatusCode;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn load() -> SMTP {
    dotenvy::dotenv().expect("\".env\" file is missing");

    let host: String =
        dotenvy::var("SMTP_HOST").expect("\"SMTP_HOST\"  is missing from \".env\" file");

    let username: String =
        dotenvy::var("SMTP_USERNAME").expect("\"SMTP_USERNAME\" is missing from \".env\" file");

    let password: String =
        dotenvy::var("SMTP_PASSWORD").expect("\"SMTP_PASSWORD\" is missing from \".env\" file");

    let credentials: Credentials = Credentials::new(username, password);

    let from: String =
        dotenvy::var("SMTP_FROM").expect("\"SMTP_FROM\" is missing from \".env\" file");

    let from: Mailbox = from.parse().expect("\"SMTP_FROM\" is invalid");

    SMTP {
        host,
        credentials,
        from,
    }
}

#[derive(Clone)]
pub struct SMTP {
    host: String,
    credentials: Credentials,
    from: Mailbox,
}

impl SMTP {
    pub fn send_code(self, to: String, code: i64) -> Result<StatusCode, StatusCode> {
        let to: Mailbox = match to.parse() {
            Ok(to) => to,
            Err(e) => {
                eprintln!("smtp > send_code > {e}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let message: Message = match Message::builder()
            .from(self.from)
            .to(to)
            .subject("sumire: code")
            .header(ContentType::TEXT_PLAIN)
            .body(code.to_string())
        {
            Ok(message) => message,
            Err(e) => {
                eprintln!("smtp > send_code > {e}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let mailer: SmtpTransport = SmtpTransport::relay(&self.host)
            .unwrap()
            .credentials(self.credentials)
            .build();

        match mailer.send(&message) {
            Ok(_) => Ok(StatusCode::OK),
            Err(e) => {
                eprintln!("smtp > send_code > {e}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
