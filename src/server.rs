pub mod sts {
    tonic::include_proto!("sts");
}

mod jwt;

use std::{fs, error::Error};

use clap::Parser;
use tonic::{transport::Server, Request, Response, Status};
use sts::{sts_server::Sts, TokenRequest, TokenResponse};
use jsonwebtoken::{Header, Algorithm, EncodingKey, encode};

use sts::sts_server::StsServer;


const DEFAULT_LISTEN_PORT: u16 = 0xB47E;
const DEFAULT_AUDIENCE: &'static str = "internal_service";

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
}

#[derive(Debug)]
struct TokenService {
    config: Config,
    cert: openssl::x509::X509,
}

#[tonic::async_trait]
impl Sts for TokenService {
    async fn get_token( &self, request: Request<TokenRequest> ,) ->  Result<Response<TokenResponse>, Status> {
        let claims = jwt::Claims::new();
        let mut header = Header::new(Algorithm::HS512);
        header.kid = Some("762c640e-d333-4fc3-a95e-f74370124621".to_owned());
        let result = encode(&header, 
            &claims, &EncodingKey::from_secret("test secret - must replace".as_bytes()));
        match result {
            Ok(token) => {
                let response = TokenResponse{ duration: request.into_inner().duration, token };
                Ok(Response::new(response))    
            }
            _=> Err(Status::internal("unable to create token"))
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn  std::error::Error>> {

    let config = Config::parse();

    let cert = load_cert(&config.cert_file)?;

    let addr = ("[::1]:".to_string()+&(DEFAULT_LISTEN_PORT.to_string())).parse()?;
    let token_service = TokenService{config};

    Server::builder()
        .add_service(StsServer::new(token_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn load_cert(cert_file: &str) -> Result<openssl::x509::X509, Box<dyn std::error::Error>>{
    let path = std::path::Path::new(cert_file);
    let mut x509_cert: Option<openssl::x509::X509> = None;

    match fs::read(path) {
        Ok(vec_bytes) => {
            match openssl::x509::X509::from_pem(&vec_bytes) {
                Ok(cert) => x509_cert = Some(cert),
                Err(e) => return Err(Box::new(e)),
            }
        }
        Err(e) =>  {
            println!("unable to read file");
            return Err(Box::new(e));
        }
    };
    Ok(x509_cert.unwrap())
}


fn load_private_key(key_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    match fs::read(key_path) {
        Ok(pem) => {
            match openssl::pkey::PKey::private_key_from_pem(&pem) {
                Ok(priv_key) => {

                }
                Err(e) => {

                }
            }

        }
        Err(e) => {

        }
    }
    Ok(())
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