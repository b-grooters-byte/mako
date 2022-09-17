use tonic_build::compile_protos;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./proto/sts.proto")?;

    Ok(())
}
