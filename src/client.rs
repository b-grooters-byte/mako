pub mod sts {
    tonic::include_proto!("sts");
}

use sts::sts_client::StsClient;
use sts::TokenRequest;
use tonic::{codegen::http::uri::Authority, transport::Uri};

const DEFAULT_CONNECT_PORT: u16 = 0xB47E;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port_str = &DEFAULT_CONNECT_PORT.to_string();
    let authority = ("[::1]:".to_string() + port_str)
        .parse::<Authority>()
        .unwrap();
    println!("Authority {:?}", authority);
    let uri = Uri::builder()
        .scheme("http")
        .authority(authority)
        .path_and_query("")
        .build();
    let channel = tonic::transport::Channel::builder(uri.unwrap())
        .connect()
        .await?;
    let mut client = StsClient::new(channel);
    let request = tonic::Request::new(TokenRequest { duration: 10 });
    let response = client.get_token(request).await?.into_inner();
    println!("RESPONSE = {:?}", response);

    Ok(())
}
