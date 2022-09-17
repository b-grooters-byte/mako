pub mod sts {
    tonic::include_proto!("sts");
}

use tonic::{transport::Server, Request, Response, Status};
use sts::{sts_server::Sts, TokenRequest, TokenResponse};

use sts::sts_server::StsServer;

#[derive(Debug, Default)]
struct TokenService {}

#[tonic::async_trait]
impl Sts for TokenService {
    async fn get_token( &self, request: Request<TokenRequest> ,) ->  Result<Response<TokenResponse>, Status> {
        let response = TokenResponse{ duration: request.into_inner().duration, token: "A Token".to_string() };
        Ok(Response::new(response))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn  std::error::Error>> {
    let addr = "[::1]:20010".parse()?;
    let token_service = TokenService::default();

    Server::builder()
        .add_service(StsServer::new(token_service))
        .serve(addr)
        .await?;

    Ok(())
}