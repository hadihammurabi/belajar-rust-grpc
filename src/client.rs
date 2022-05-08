use belajar_rust_grpc::api::hello::{
  None,
  hello_client::HelloClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloClient::connect("http://[::1]:50051").await?;
    let req = tonic::Request::new(None{});
    let res = client.say(req).await?;

    println!("RESPONSE: {}", res.into_inner().message);

    Ok(())
}
