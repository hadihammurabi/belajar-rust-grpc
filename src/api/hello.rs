use tonic::{
    Request,
    Response,
    Status,
};
use crate::api::hello_gen::{
    None,
    HelloReply,
    hello_server::{
        Hello,
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
