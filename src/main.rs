use clap::Parser;
use jsonwebtoken::{Header, Algorithm, EncodingKey, encode};
use serde::{Serialize, Deserialize};


#[derive(Parser)]
pub struct Config {
    #[clap(short, long, value_parser)]
    secret: String,
    #[clap(short,long)]
    audience: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: Option<String>,
    exp: usize,

}

fn main() {


    let config = Config::parse();

    let claims = Claims {
        aud: config.audience,
        exp: 0,

    };
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("762c640e-d333-4fc3-a95e-f74370124621".to_owned());
    let result = encode(&header, 
        &claims, &EncodingKey::from_secret(config.secret.as_ref()));
    match result {
        Ok(token) =>     println!("Token {}", token),
        Err(e) => println!("unable to create token: {}", e),
    }
}
