use std::net::SocketAddr;

use std::{
    env,
    net::{IpAddr, Ipv4Addr},
};

use hyper_services::{service::stateful_service::StatefulService, spawn_server};

#[tokio::main]
async fn main() {
    println!("Starting.");

    let args: Vec<String> = env::args().collect();

    let internal_port = match args.get(3) {
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

    let external_port = match args.get(3) {
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
        println!("Creating pixel strip manager.");
        let internal_service = PixelStripManager::new(pixel_strip, DISPLAY_FREQUENCY, conn);

        //demos::red_green_blue(conn, pixels)?;
        //demos::hue_progression(conn, pixels)?;
        //demos::rainbow_oscillation(conn, pixel_strip).unwrap();

        println!("Creating LED Command Handler.");

        let handler = LedCommandHandler::new(pixel_strip_manager);

        println!("Starting DDP Service");

        spawn_server(
            IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            service_port,
            StatefulService::create(handler),
        )
    };

    println!("Services Running");

    match event_server.await {
        Ok(_) => println!("Closed DDP Service Gracefully"),
        Err(e) => {
            println!("DDP Service Failure");
            println!("{}", e.to_string());
        }
    };
}
