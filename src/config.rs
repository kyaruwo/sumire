use tokio::net::TcpListener;

pub struct Config {
    pub tcp_listener: TcpListener,
    pub address: String,
    pub database_url: String,
    pub aes_key: String,
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

    Config {
        address,
        tcp_listener,
        database_url,
        aes_key,
    }
}
