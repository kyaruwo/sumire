use std::net::SocketAddr;

pub struct Config {
    pub socket_address: SocketAddr,
    pub database_url: String,
    pub aes_key: String,
}

pub fn load() -> Config {
    dotenvy::dotenv().expect("\".env\" file is missing");
    let socket_address: SocketAddr = dotenvy::var("SOCKET_ADDRESS")
        .expect("\"SOCKET_ADDRESS\"  is missing from \".env\" file")
        .parse()
        .expect("\"SOCKET_ADDRESS\" is invalid, either IPv4 or IPv6");
    let database_url: String =
        dotenvy::var("DATABASE_URL").expect("\"DATABASE_URL\" is missing from \".env\" file");
    let aes_key: String =
        dotenvy::var("AES_KEY").expect("\"AES_KEY\" is missing from \".env\" file");

    Config {
        socket_address,
        database_url,
        aes_key,
    }
}
