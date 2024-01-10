use tokio::net::TcpListener;

pub struct Configuration {
    pub address: String,
    pub tcp_listener: TcpListener,
    pub database_url: String,
}

pub async fn load() -> Configuration {
    dotenvy::dotenv().expect("\".env\" file is missing");

    let address: String =
        dotenvy::var("ADDRESS").expect("\"ADDRESS\" is missing from \".env\" file");

    let tcp_listener: TcpListener = TcpListener::bind(&address)
        .await
        .expect("ADDRESS is invalid or inuse");

    let database_url: String =
        dotenvy::var("DATABASE_URL").expect("\"DATABASE_URL\" is missing from \".env\" file");

    Configuration {
        address,
        tcp_listener,
        database_url,
    }
}
