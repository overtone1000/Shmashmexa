use std::net::SocketAddr;

use std::{
    env,
    net::{IpAddr, Ipv4Addr},
};

use hyper_services::service::stateless_service::StatelessService;
use hyper_services::{service::stateful_service::StatefulService, spawn_server};
use shmashmexa_backend::services::internal::InternalService;

#[tokio::main]
async fn main() {
    println!("Starting.");

    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}",args);

    let internal_port = match args.get(1) {
        Some(port) => match port.parse::<u16>() {
            Ok(port) => port,
            Err(e) => {
                eprintln!("Invalid port {}", port);
                eprintln!("{:?}", e);
                return;
            }
        },
        None => {
            eprintln!("Provide the internal port as the first argument.");
            return;
        }
    };

    let external_port = match args.get(2) {
        Some(port) => match port.parse::<u16>() {
            Ok(port) => port,
            Err(e) => {
                eprintln!("Invalid port {}", port);
                eprintln!("{:?}", e);
                return;
            }
        },
        None => {
            eprintln!("Provide the external port as the second argument.");
            return;
        }
    };

    //Create event servers
    let event_server = {

        let internal_service:StatelessService<InternalService>=StatelessService::create();

        spawn_server(
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            internal_port,
            internal_service,
        )
    };

    println!("Services Running");

    match event_server.await {
        Ok(_) => println!("Closed internal service gracefully"),
        Err(e) => {
            println!("DDP Service Failure");
            println!("{}", e.to_string());
        }
    };
}
