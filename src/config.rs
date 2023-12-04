pub struct Config {
    pub address: String,
    pub database_url: String,
    pub aes_key: String,
}

pub fn load() -> Config {
    dotenvy::dotenv().expect("\".env\" file is missing");

    let address = dotenvy::var("ADDRESS").expect("\"ADDRESS\"  is missing from \".env\" file");

    let database_url: String =
        dotenvy::var("DATABASE_URL").expect("\"DATABASE_URL\" is missing from \".env\" file");

    let aes_key: String =
        dotenvy::var("AES_KEY").expect("\"AES_KEY\" is missing from \".env\" file");

    Config {
        address,
        database_url,
        aes_key,
    }
}
