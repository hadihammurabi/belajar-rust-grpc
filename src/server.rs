use tonic::{transport::Server};

use belajar_rust_grpc::api::{
    hello_service::build_hello_server,
    calculator_service::build_calculator_server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(build_hello_server())
        .add_service(build_calculator_server())
        .serve(addr)
        .await?;

    Ok(())
}
