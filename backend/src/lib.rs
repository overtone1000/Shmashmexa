pub(crate) mod services;


use std::
    net::{IpAddr, Ipv4Addr}
;

use hyper_services::service::stateful_service::StatefulService;
use hyper_services::service::stateless_service::StatelessService;
use hyper_services::{ConnectionProperties};
use rustls::pki_types::pem::PemObject;

use crate::services::external::ExternalService;
use crate::services::internal::InternalService;

#[derive(Debug)]
pub struct InitializationParameters
{
    internal_service_static_directory:String,
    config_static_directory:String,
    internal_port:u16,
    external_port:u16
}

impl InitializationParameters
{
    pub fn new(internal_service_static_directory:&str, config_static_directory:&str, internal_port:u16, external_port:u16)->InitializationParameters
    {
        InitializationParameters { 
            internal_service_static_directory:internal_service_static_directory.to_string(),
            config_static_directory:config_static_directory.to_string(),
            internal_port, 
            external_port 
        }
    }
}

pub async fn start_and_run(params:InitializationParameters) {
    loop {
        
        println!("Starting services.");

        //Create event servers
        let internal_service = {

            let handler = InternalService::new(&params);
            let service=StatefulService::create(handler);
            
            service.start(
                IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                params.internal_port,
                ConnectionProperties{
                    with_upgrades:true,
                    tls:None
                }
            )
        };

        let external_service = {

            let service:StatelessService<ExternalService>=StatelessService::create();

            match rcgen::generate_simple_self_signed(["10.10.10.154".to_string(),"127.0.0.1".to_string(),"localhost".to_string()])
            {
                Ok(keypair)=>{
                    
                    let certs =  vec![rustls::pki_types::CertificateDer::from(keypair.cert)];
                    let keys = rustls::pki_types::PrivateKeyDer::from(keypair.signing_key);

                    let certs:hyper_services::TlsCerts = hyper_services::TlsCerts{
                        certs,
                        keys
                    };
                    service.start(
                        IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                        params.external_port,
                        ConnectionProperties{
                            with_upgrades:false,
                            tls:Some(certs)
                        }
                    )
                },
                Err(e)=>{
                    panic!("Couldn't create certificates. {:?}",e);
                }
            }
        };

        println!("Services created.");

        match tokio::try_join!(internal_service, external_service)
        {
            Ok(_) => println!("Services closed gracefully."),
            Err(e) => {
                println!("Service Failure");
                println!("{}", e.to_string());
            }
        }
    }
}
