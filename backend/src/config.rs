use dotenvy;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::{env, fs, io};

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database_url: String,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub expires_in: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        match dotenvy::from_path("../.env") {
            Ok(_) => {},
            Err(_) => { dotenvy::dotenv().ok(); },
        }

        let private_key_path = env::var("JWT_PRIVATE_KEY_PATH")
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Environment variable JWT_PRIVATE_KEY_PATH not set"))?;
        let private_key = fs::read(&private_key_path)
            .map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Failed to read private key file: {}, path: {}", e, private_key_path)))?;
        let jwt_private_key = EncodingKey::from_rsa_pem(&private_key)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse JWT private key: {}", e)))?;

        let public_key_path = env::var("JWT_PUBLIC_KEY_PATH")
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Environment variable JWT_PUBLIC_KEY_PATH not set"))?;
        let public_key = fs::read(&public_key_path)
            .map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Failed to read public key file: {}, path: {}", e, public_key_path)))?;
        let jwt_public_key = DecodingKey::from_rsa_pem(&public_key)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse JWT public key: {}", e)))?;


        let config = Config {
            server: ServerConfig {
                host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("PORT")
                    .ok()
                    .and_then(|p| p.parse::<u16>().ok())
                    .unwrap_or(3000),
            },
            database_url: env::var("DATABASE_URL")
                .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Environment variable DATABASE_URL not set"))?,
            jwt: JwtConfig {
                encoding_key: jwt_private_key,
                decoding_key: jwt_public_key,
                expires_in: env::var("JWT_EXPIRES_IN")
                    .ok()
                    .and_then(|p| p.parse::<i64>().ok()).unwrap_or(86400),
            }
        };
        Ok(config)
    }
}