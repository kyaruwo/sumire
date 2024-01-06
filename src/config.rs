use axum::http::StatusCode;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tokio::net::TcpListener;

pub struct Config {
    pub tcp_listener: TcpListener,
    pub address: String,
    pub database_url: String,
    pub aes_key: String,
    pub smtp: SMTP,
}

pub async fn load() -> Config {
    dotenvy::dotenv().expect("\".env\" file is missing");

    let address: String =
        dotenvy::var("ADDRESS").expect("\"ADDRESS\"  is missing from \".env\" file");

    let tcp_listener: TcpListener = TcpListener::bind(&address)
        .await
        .expect("\"ADDRESS\" is invalid or inuse");

    let database_url: String =
        dotenvy::var("DATABASE_URL").expect("\"DATABASE_URL\" is missing from \".env\" file");

    let aes_key: String =
        dotenvy::var("AES_KEY").expect("\"AES_KEY\" is missing from \".env\" file");

    let smtp: SMTP = SMTP::load();

    Config {
        address,
        tcp_listener,
        database_url,
        aes_key,
        smtp,
    }
}

#[derive(Clone)]
pub struct SMTP {
    host: String,
    credentials: Credentials,
    from: Mailbox,
}

impl SMTP {
    fn load() -> SMTP {
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

    pub fn send_code(self, to: String, code: u64) -> Result<(), StatusCode> {
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
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("smtp > send_code > {e}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
