use crate::api::calculator::{
  calculator_server::{Calculator, CalculatorServer},
  CalculatorReply, CalculatorRequest,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
  async fn add(
    &self,
    request: Request<CalculatorRequest>,
  ) -> Result<Response<CalculatorReply>, Status> {
    println!(
      "Received request from: {:?}",
      request
        .metadata()
        .get("user-agent")
        .ok_or("unknown")
        .unwrap()
    );

    let data = request.into_inner();
    let response = CalculatorReply {
      result: data.op1 + data.op2,
    };

    Ok(Response::new(response))
  }

  async fn sub(
    &self,
    request: Request<CalculatorRequest>,
  ) -> Result<Response<CalculatorReply>, Status> {
    println!(
      "Received request from: {:?}",
      request
        .metadata()
        .get("user-agent")
        .ok_or("unknown")
        .unwrap()
    );

    let data = request.into_inner();
    let response = CalculatorReply {
      result: data.op1 - data.op2,
    };

    Ok(Response::new(response))
  }

  async fn mul(
    &self,
    request: Request<CalculatorRequest>,
  ) -> Result<Response<CalculatorReply>, Status> {
    println!(
      "Received request from: {:?}",
      request
        .metadata()
        .get("user-agent")
        .ok_or("unknown")
        .unwrap()
    );

    let data = request.into_inner();
    let response = CalculatorReply {
      result: data.op1 * data.op2,
    };

    Ok(Response::new(response))
  }

  async fn div(
    &self,
    request: Request<CalculatorRequest>,
  ) -> Result<Response<CalculatorReply>, Status> {
    println!(
      "Received request from: {:?}",
      request
        .metadata()
        .get("user-agent")
        .ok_or("unknown")
        .unwrap()
    );

    let data = request.into_inner();
    let response = CalculatorReply {
      result: data.op1 / data.op2,
    };

    Ok(Response::new(response))
  }
}

pub fn build_calculator_server() -> CalculatorServer<CalculatorService> {
  let hello_service = CalculatorService::default();
  CalculatorServer::new(hello_service)
}
