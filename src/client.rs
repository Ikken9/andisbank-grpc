use hyper_util::rt::TokioExecutor;
use tonic_web::GrpcWebClientLayer;
use crate::hello_world::GetLoansRequest;
use crate::hello_world::loan_service_client::LoanServiceClient;

pub mod hello_world {
    tonic::include_proto!("loan");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Must use hyper directly...
    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build_http();

    let svc = tower::ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(client);

    let mut client = LoanServiceClient::with_origin(svc, "http://127.0.0.1:3000".try_into()?);

    let request = tonic::Request::new(GetLoansRequest {

    });

    let response = client.get_loans(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}