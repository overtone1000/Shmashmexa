use std::collections::HashMap;

use has_mqtt::{component::HomeAssistantDeviceComponent, device::HomeAssistantDeviceConfiguration, mqtt_client::{DEFAULT_DISCOVERY_PREFIX, HASMQTTClient}, platform::switch::state::SwitchState};

pub async fn get_has_client(kiosk_uid:u64)->HASMQTTClient
{

    let mut cmps_hm:HashMap<String,HomeAssistantDeviceComponent> = HashMap::new();

    let mut add_cmp = |kvp:(String,HomeAssistantDeviceComponent)|->(){
        cmps_hm.insert(kvp.0,kvp.1);
    };

    add_cmp(monitor_switch(kiosk_uid));

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
        HomeAssistantDeviceComponent::new_switch(
            "faux_show_monitor",
            "Faux Show Monitor",
            Box::new(handle_state_change)
        )
    )
}

fn remote_url_set()->(String,HomeAssistantDeviceComponent)
{
    (
        "url_set".to_string(),
        HomeAssistantDeviceComponent::
    )
}