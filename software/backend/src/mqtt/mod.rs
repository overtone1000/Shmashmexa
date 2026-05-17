use std::collections::HashMap;

use has_mqtt::{component::HomeAssistantDeviceComponent, device::HomeAssistantDeviceConfiguration, mqtt_client::{DEFAULT_DISCOVERY_PREFIX, HASMQTTClient}, platform::{switch::{component::Switch, state::SwitchState}, text::component::Text}};


use crate::{commands::{ChangeDashData, Command}, services::{external::external_core::ExternalCore, internal::{self, InternalService}}};

#[derive(Debug)]
pub struct MQTTConfiguration
{
    pub id:String,
    pub name:String,
    pub origin_name:String,
    pub origin_sw:String,
    pub client_id:String,
    pub server_url:String,
    pub server_port:u16,
    pub object_id:String,
    pub discovery_prefix:String
}

pub async fn get_has_client(external_core:ExternalCore, config:&MQTTConfiguration, kiosk_uid:u64)->HASMQTTClient
{

    let mut cmps_hm:HashMap<String,HomeAssistantDeviceComponent> = HashMap::new();

    let mut add_cmp = |kvp:(String,HomeAssistantDeviceComponent)|->(){
        cmps_hm.insert(kvp.0,kvp.1);
    };

    add_cmp(monitor_switch(&config.id, &config.name, kiosk_uid));
    add_cmp(remote_url_set(&config.id, &config.name, external_core));

    let device=HomeAssistantDeviceConfiguration::new(
        config.id.to_string(),
        config.name.to_string(),
        config.origin_name.to_string(),
        config.origin_sw.to_string(),
        cmps_hm
    );


    HASMQTTClient::start(
        &config.client_id,
        &config.server_url,
        config.server_port,
        &config.discovery_prefix,
        &config.object_id,
        device
    ).await
}

fn monitor_switch(device_id:&str, device_name:&str, kiosk_uid:u64)->(String,HomeAssistantDeviceComponent)
{
    let handle_state_change =move |state:SwitchState|->Option<SwitchState>
    {
        match crate::device::set_screen_state(state.as_bool(),&kiosk_uid)
        {
            Ok(_)=>Some(state),
            Err(e)=>{
                eprintln!("Error setting screen state. {:?}",e);  
                None
            }
        }
    };

    (
        "monitor".to_string(),
        Switch::new(
            device_id,
            device_name,
            "monitor",
            "Monitor",
            Box::new(handle_state_change)
        )
    )
}

fn remote_url_set(device_id:&str, device_name:&str, external_core:ExternalCore)->(String,HomeAssistantDeviceComponent)
{
    let handle_state_change =move |new_url:String|->Option<String>
    {
        match str::parse::<hyper::Uri>(&new_url)
        {
            Ok(_) => {

                //ChangeDash
                let command:Command=Command::ChangeDashUrl(new_url.to_string());
                match external_core.command_sender.send(command)
                {
                    Ok(_) => Some(new_url),
                    Err(e) => {
                        eprintln!("{:?}",e);
                        None
                    },
                }
            },
            Err(e) => {
                eprintln!("{:?}",e);
                None
            },
        }
    };

    (
        "url_set".to_string(),
        Text::new(
            device_id,
            device_name,
            "url",
            "URL",
            Box::new(handle_state_change)
        )
    )
}