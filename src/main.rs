use manage::startup::run;
use manage::configuration::get_configuration;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).unwrap();
    run(listener)?.await
}
