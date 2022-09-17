pub mod sts {
    tonic::include_proto!("sts");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}