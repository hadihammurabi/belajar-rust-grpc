use tonic::{
    Request,
    Response,
    Status,
};
use crate::api::hello::{
    None,
    HelloReply,
    hello_server::{
        Hello,
        HelloServer,
    },
};

#[derive(Debug, Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn say(
        &self,
        request: Request<None>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Received request from: {:?}", request.metadata().get("user-agent").ok_or("unknown").unwrap());

        let response = HelloReply {
            message: "Halooo...!!".into(),
        };

        Ok(Response::new(response))
    }
}

pub fn build_hello_server() -> HelloServer::<HelloService> {
    let hello_service = HelloService::default();
    HelloServer::new(hello_service)
}
