use tonic::{transport::Server};

use belajar_rust_grpc::api::{
    hello::HelloService,
    hello_gen::hello_server::HelloServer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let hello_service = HelloService::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(HelloServer::new(hello_service))
        .serve(addr)
        .await?;

    Ok(())
}
