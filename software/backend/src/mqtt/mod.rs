use std::collections::HashMap;

use has_mqtt::{component::HomeAssistantDeviceComponent, device::HomeAssistantDeviceConfiguration, mqtt_client::{DEFAULT_DISCOVERY_PREFIX, HASMQTTClient}, platform::{switch::{component::Switch, state::SwitchState}, text::component::Text}};


use crate::{commands::{ChangeDashData, Command}, services::{external::external_core::ExternalCore, internal::{self, InternalService}}};

pub async fn get_has_client(kiosk_uid:u64, external_core:ExternalCore)->HASMQTTClient
{

    let mut cmps_hm:HashMap<String,HomeAssistantDeviceComponent> = HashMap::new();

    let mut add_cmp = |kvp:(String,HomeAssistantDeviceComponent)|->(){
        cmps_hm.insert(kvp.0,kvp.1);
    };

    add_cmp(monitor_switch(kiosk_uid));
    add_cmp(remote_url_set(external_core));

    let device=HomeAssistantDeviceConfiguration::new(
        "faux_show".to_string(),
        "Faux Show".to_string(),
        "Tyler Moore".to_string(),
        "0.1.0".to_string(),
        cmps_hm
    );


    HASMQTTClient::start(
        "faux_show_client",
        "10.10.10.10",
        1883,
        DEFAULT_DISCOVERY_PREFIX,
        "faux_show",
        device
    ).await
}

fn monitor_switch(kiosk_uid:u64)->(String,HomeAssistantDeviceComponent)
{
    let handle_state_change =move |state:SwitchState|->SwitchState
    {
        match crate::device::set_screen_state(state.as_bool(),&kiosk_uid)
        {
            Ok(_)=>state,
            Err(e)=>{
                eprintln!("Error setting screen state. {:?}",e);  
                !state
            }
        }
    };

    (
        "monitor".to_string(),
        Switch::new(
            "faux_show_monitor",
            "Faux Show Monitor",
            Box::new(handle_state_change)
        )
    )
}

fn remote_url_set(external_core:ExternalCore)->(String,HomeAssistantDeviceComponent)
{
    let handle_state_change =move |state:String|->String
    {
        //ChangeDash
        let command:Command=Command::ChangeDash(
            ChangeDashData{
                index: todo!(),
            }
        );
        external_core.command_sender.send(ChangeDash)
    };

    (
        "url_set".to_string(),
        Text::new(
            "faux_show_url",
            "Faux Show URL",
            Box::new(handle state_change)
        )
    )
}