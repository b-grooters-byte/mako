mod jwt;

use clap::Parser;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

#[derive(Parser)]
pub struct Config {
    #[clap(short, long, value_parser)]
    secret: String,
    #[clap(short, long)]
    audience: Option<String>,
}

fn main() {
    let config = Config::parse();
    let claims = jwt::Claims::new();
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("762c640e-d333-4fc3-a95e-f74370124621".to_owned());
    let result = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    );
    match result {
        Ok(token) => println!("Token {}", token),
        Err(e) => println!("unable to create token: {}", e),
    }
}
