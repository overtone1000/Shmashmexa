use std::{collections::VecDeque, sync::Arc, time::{Duration, Instant}};

use hyper::{body::Incoming, Method, Request, Response};
use hyper_services::{
    commons::HandlerResult, request_processing::get_request_body_as_string, response_building::{bad_request, box_existing_full, box_existing_response, bytes_to_boxed_body}, service::stateful_service::StatefulHandler
};

use hyper_tungstenite::{HyperWebsocket, WebSocketStream, tungstenite};
use tokio::sync::Mutex;
use tungstenite::Message;
use websocket::WebSocketStreamNext;

use crate::commands::Command;

const CONFIG_PREFACE:&str="/config";

#[derive(Clone)]
pub struct InternalService {
    internal_service_static_directory:String,
    config_static_directory:String,
    commands:Arc<Mutex<VecDeque<Command>>>
}

impl InternalService
{
    pub fn new(initialization_parameters:&crate::InitializationParameters)->InternalService
    {
        InternalService { 
            internal_service_static_directory: initialization_parameters.internal_service_static_directory.clone(),
            config_static_directory: initialization_parameters.config_static_directory.clone(),
            commands:Arc::new(Mutex::new(VecDeque::new()))
        }
    }

    pub async fn push_command(&mut self, command:Command)->()
    {
        let mut commands = self.commands.lock().await;
        commands.push_back(command);
        //println!("Added: Commands now contains {} commands.",commands.len());
    }

    async fn handle_websocket(self, websocket: HyperWebsocket) -> () {       

        use futures_util::stream::StreamExt;
        use futures_util::SinkExt;

        println!("Serving websocket");
        let mut websocketstream = match websocket.await{
            Ok(websocketstream) => websocketstream,
            Err(e) => {
                eprintln!("Websocket error: {:?}",e);
                return;
            },
        };

        let target_wait = Duration::from_millis(100);

        loop
        {
            let loop_start = std::time::Instant::now();

            println!("Awaiting next, but this blocks until a message arrives.");
            match websocketstream.next().
            {
                Some(stream_next) => {    
                    match stream_next {
                        Ok(msg)=>{
                            match msg
                            {
                                Message::Text(msg) => {
                                    println!("Received text message: {msg}");
                                    match websocketstream.send(Message::text("Ok")).await
                                    {
                                        Ok(_)=>(),
                                        Err(e)=>{
                                            eprintln!("Websocket error: {:?}",e);
                                        }
                                    }
                                },
                                Message::Ping(_)=>println!("Ping"),
                                Message::Pong(_)=>println!("Pong"),
                                _=>() //Ignore all other message types.
                            }
                        },
                        Err(e) => {
                            eprintln!("Websocket error: {:?}",e);
                            return;
                        },
                    }            
                },
                None => (),
            }

            println!("Processing commands.");
            let mut commands = self.commands.lock().await;
            while commands.len()>0
            {
                println!("Commands to process.");
                match commands.pop_front()
                {
                    Some(command)=>{
                        match serde_json::to_string(&command)
                        {
                            Ok(command_as_string)=>{
                                //println!("Sending command.");
                                match websocketstream.send(Message::text(command_as_string)).await
                                {
                                    Ok(_)=>{
                                        println!("Command sent via websocket.");
                                    },
                                    Err(e)=>{
                                        eprintln!("Websocket error: {:?}",e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Couldn't deserialize command. {:?}",e);
                            },
                        }
                    },
                    None=>()
                }
            }

            let wait = match loop_start.checked_add(target_wait)
            {
                Some(wait)=>wait.duration_since(Instant::now()),
                None=>target_wait
            };

            //tokio::time::sleep(wait);
        }
    }
}

impl StatefulHandler for InternalService {
    async fn handle_request(self:Self, request: Request<Incoming>) -> HandlerResult {

        match hyper_tungstenite::is_upgrade_request(&request) {
            true=>{
                let (response, websocket) = hyper_tungstenite::upgrade(request, None)?;
            
                println!("Received websocket request. Response is {:?}", response);
                // Spawn a task to handle the websocket connection.
                tokio::spawn(async move {
                    self.handle_websocket(websocket).await
                });

                // Return the response so the spawned future can continue.
                let boxed_response=box_existing_response(response);
                println!("Boxed response is {:?}", boxed_response);
                Ok(boxed_response)
            },
            false=>{
                let (parts, incoming) = request.into_parts();
                        
                match parts.method {
                    Method::POST => {
                        let body= match get_request_body_as_string(incoming).await
                        {
                            Ok(body)=>body,
                            Err(e)=>{
                                eprintln!("Couldn't get request body. {:?}",e);
                                return Ok(bad_request());
                            }
                        };

                        println!("Received POST {:?} with body {:?}",parts.uri, body);

                        Ok(Response::new(bytes_to_boxed_body("Ok")))
                    },
                    Method::GET => {
                        //println!("Received GET for {:?}",parts.uri);                       
                        
                        if parts.uri.path().starts_with(CONFIG_PREFACE){
                            let final_path=parts.uri.path().split_at(CONFIG_PREFACE.len()).1;
                            //println!("Serving config {:?} - {:?}",&self.config_static_directory,final_path);
                            hyper_services::response_building::send_file(&self.config_static_directory,final_path).await
                        }
                        else {
                            //println!("Serving base.");
                            hyper_services::response_building::send_file(&self.internal_service_static_directory,parts.uri.path()).await
                        }
                    },
                    method=>{
                        eprintln!("Received unexpected method {:?}",method);
                        Ok(bad_request())
                    }
                }
            }   
        }
    }
}


