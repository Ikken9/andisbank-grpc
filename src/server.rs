use tonic::{transport::Server, Request, Response, Status};
use crate::loan_server::{GetLoansRequest, GetLoansResponse, Loan};
use crate::loan_server::loan_service_server::{LoanService, LoanServiceServer};

pub mod loan_server {
    tonic::include_proto!("loan");
}

#[derive(Debug, Default)]
pub struct MyLoanService {}


#[tonic::async_trait]
impl LoanService for MyLoanService {
    async fn get_loans(&self, request: Request<GetLoansRequest>) -> Result<Response<GetLoansResponse>, Status> {
        println!("Got a request: {:?}", request);
        let reply = GetLoansResponse {
            loans: vec![
                Loan {
                    id: 1,
                    user_id: 12345,
                    loan_type_id: 1,
                    amount: 15000.0,
                    currency: "USD".to_owned(),
                    term_months: 36,
                    interest_rate: 5.5,
                    monthly_payment: 452.12,
                    balance: 13500.0,
                    status: "active".to_owned(),
                    start_date: "2024-09-04T00:00:00Z".to_owned(),
                    end_date: "2027-09-04T00:00:00Z".to_owned()
                }
            ]
        };

        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000".parse().unwrap();
    let loan_dispatcher = MyLoanService::default();
    let loan_server = LoanServiceServer::new(loan_dispatcher);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(loan_server))
        .serve(addr)
        .await?;

    Ok(())
}