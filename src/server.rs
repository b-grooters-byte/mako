pub mod sts {
    tonic::include_proto!("sts");
}

mod jwt;

use std::{error::Error, fmt::Display, fs};

use base64::CharacterSet;
use clap::Parser;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use openssl::{hash::MessageDigest, pkey::Id};
use sts::{sts_server::Sts, TokenRequest, TokenResponse};
use tonic::{transport::Server, Request, Response, Status};

use sts::sts_server::StsServer;

const DEFAULT_LISTEN_PORT: u16 = 0xB47E;
const DEFAULT_AUDIENCE: &'static str = "internal_service";
const DEFAULT_KEY_ID: &'static str = "MAKO_STS_KEY";

#[derive(Debug)]
struct KeyError {
    message: String,
}

impl KeyError {
    pub(crate) fn new(message: &str) -> Self {
        KeyError { message: message.to_owned() }
    }
}

impl Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.message)) 
    }
}

impl std::error::Error for KeyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[derive(Debug)]
pub struct UnsupportedKeyType {
    key_type: openssl::pkey::Id,
}

impl Display for UnsupportedKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Unsupported key type: {:?}", self.key_type))
    }
}

impl std::error::Error for UnsupportedKeyType {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[derive(Debug, Parser)]
#[clap(version, about)]
struct Config {
    #[clap(short, long)]
    max_duration: Option<u32>,
    /// the default duration of tokens issued by the STS. Defaults to 15 minutes
    #[clap(short, long)]
    default_duration: Option<u32>,
    /// the listen port for gRPC connections
    #[clap(short, long, default_value_t = DEFAULT_LISTEN_PORT)]
    listen_port: u16,
    /// certificate file location for STS JWT signing cert
    #[clap(short, long)]
    cert_file: String,
    #[clap(short, long = "priv")]
    priv_key_file: String,
    /// Optional key ID. 
    #[clap(short, long, default_value_t=DEFAULT_KEY_ID.to_owned())]
    key_id: String
}

struct TokenService {
    config: Config,
    cert: openssl::x509::X509,
    thumbprint: String,
    key: EncodingKey,
    key_id: String,
}

impl TokenService {
    pub(crate) fn new(
        config: Config,
        cert: openssl::x509::X509,
        key: EncodingKey,
        key_id: String,
    ) -> Result<Self, KeyError> {
        // create the thumbprint for the current key
        let thumbprint: String;
        match cert.digest(MessageDigest::sha256()) {
            Ok(digest) => {
                thumbprint = base64::encode_config(
                    &digest,
                    base64::Config::new(CharacterSet::UrlSafe, false),
                );
            }
            Err(_e) => return Err(KeyError::new("unable to create certificate digest")),
        };
        Ok(TokenService { config, cert, thumbprint, key, key_id })
    }
}

#[tonic::async_trait]
impl Sts for TokenService {
    async fn get_token(
        &self,
        request: Request<TokenRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        let claims = jwt::Claims::new(DEFAULT_AUDIENCE);
        let mut header = Header::new(Algorithm::RS256);
        header.x5t_s256 = Some(self.thumbprint.clone());
        header.kid = Some(self.key_id.clone());
        let result = encode(&header, &claims, &self.key);
        match result {
            Ok(token) => {
                let response = TokenResponse {
                    duration: request.into_inner().duration,
                    token,
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("unable to create token {:?}", e))),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    let cert = load_cert(&config.cert_file)?;
    let key = load_private_key(&config.priv_key_file)?;
    let addr = ("[::1]:".to_string() + &(DEFAULT_LISTEN_PORT.to_string())).parse()?;
    let current_key_id = config.key_id.clone();
    let token_service = TokenService::new(
        config,
        cert,
        key,
        current_key_id,
    )?;


    Server::builder()
        .add_service(StsServer::new(token_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn load_cert(cert_file: &str) -> Result<openssl::x509::X509, Box<dyn std::error::Error>> {
    let path = std::path::Path::new(cert_file);
    let mut x509_cert: Option<openssl::x509::X509> = None;

    match fs::read(path) {
        Ok(vec_bytes) => match openssl::x509::X509::from_pem(&vec_bytes) {
            Ok(cert) => x509_cert = Some(cert),
            Err(e) => return Err(Box::new(e)),
        },
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    Ok(x509_cert.unwrap())
}

fn load_private_key(key_path: &str) -> Result<EncodingKey, Box<dyn std::error::Error>> {
    let mut key: Option<EncodingKey> = None;
    match fs::read(key_path) {
        Ok(pem) => match openssl::pkey::PKey::private_key_from_pem(&pem) {
            Ok(priv_key) => match priv_key.id() {
                Id::RSA => match EncodingKey::from_rsa_pem(&pem) {
                    Ok(key) => Ok(key),
                    Err(e) => Err(Box::new(e)),
                },
                Id::EC => match EncodingKey::from_ec_pem(&pem) {
                    Ok(key) => Ok(key),
                    Err(e) => Err(Box::new(e)),
                },
                (id) => Err(Box::new(UnsupportedKeyType { key_type: id })),
            },
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => {
            return Err(Box::new(e));
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_load_cert() {
        let cert_file = "test/server.test.crt";
        let result = load_cert(cert_file);
        assert!(result.is_ok());
    }
}
