use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct Config {
    _hmac_key: String,
    pub database_url: String,

    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub default_session_lenght: Duration,
}

impl Config {
    pub fn new(database_url: String, hmac_key: String, default_session_lenght: Duration) -> Config {
        let encoding_key = EncodingKey::from_secret(hmac_key.as_bytes());
        let decoding_key = DecodingKey::from_secret(hmac_key.as_bytes());

        Config {
            _hmac_key: hmac_key,
            database_url,
            encoding_key,
            decoding_key,
            default_session_lenght,
        }
    }
}
