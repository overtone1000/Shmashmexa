//https://www.home-assistant.io/integrations/mqtt/

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

//see component types, //https://www.home-assistant.io/integrations/mqtt/
pub const SWITCH_COMPONENT:&str="switch";

#[derive(Serialize,Deserialize,Debug,PartialEq)]
struct Device
{
    ids:String, //mandatory
    name:String, //mandatory
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
struct Origin
{
    name:String, //mandatory
    sw:String //mandatory

}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct HomeAssistantDeviceConfiguration
{
    dev:Device,
    o:Origin,
    state_topic:String,
    qos:u16, //should be 2 always
    cmps:HashMap<String,HomeAssistantDeviceComponent>
}

impl HomeAssistantDeviceConfiguration
{
    pub fn new(
        device_id:String,
        device_name:String,
        origin_name:String,
        origin_sw:String,
        state_topic:String,
        cmps:HashMap<String,HomeAssistantDeviceComponent>
    )->HomeAssistantDeviceConfiguration
    {
        HomeAssistantDeviceConfiguration{
            dev:Device{
                ids:device_id,
                name:device_name
            },
            o:Origin{
                name:origin_name,
                sw:origin_sw
            },
            state_topic,
            qos:2,
            cmps
        }
    }


    pub fn to_json(&self)->String
    {
        serde_json::to_string(self).expect("Should serialize.")
    }
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct HomeAssistantDeviceComponent_Empty
{
    p:String, //platform, mandatory, for example "switch" https://www.home-assistant.io/integrations/switch.mqtt/
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct HomeAssistantDeviceComponent_Switch
{
    p:String, //platform, mandatory, for example "switch" https://www.home-assistant.io/integrations/switch.mqtt/
    //device_class:String, //device class, optional
    command_topic:String, //mandatory, set to none to remove device
    unique_id:String //mandatory with device discovery, set to none to remove device
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
#[serde(untagged)]
pub enum HomeAssistantDeviceComponent
{
    Empty(HomeAssistantDeviceComponent_Empty),
    Switch(HomeAssistantDeviceComponent_Switch)
}

impl HomeAssistantDeviceComponent
{
    pub fn new_empty(
        platform:&str, 
    )->HomeAssistantDeviceComponent
    {
        HomeAssistantDeviceComponent::Empty(
            HomeAssistantDeviceComponent_Empty{
                p:platform.to_string(),
            }
        )
    }

    pub fn new_switch(
        platform:&str, 
        //device_class:String //optional
        command_topic:&str,
        unique_id:&str
    )->HomeAssistantDeviceComponent
    {
        HomeAssistantDeviceComponent::Switch(
            HomeAssistantDeviceComponent_Switch{
                p:platform.to_string(),
                command_topic:command_topic.to_string(),
                unique_id:unique_id.to_string()
            }
        )
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    fn check_serialization(device_config: &HomeAssistantDeviceConfiguration) {
        println!("Serialization test:");
        let serialized = device_config.to_json();
        println!("   {}", serialized);
        let deserialized: HomeAssistantDeviceConfiguration = serde_json::from_str(&serialized).expect("Should deserialize.");
        println!("   {:?}", deserialized);

        //The results won't be equal because they're untagged.
        assert_ne!(*device_config,deserialized)        
    }

    #[test]
    fn serialization() {

        let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();
        cmps.insert(
            "test_component_1".to_string(),
            HomeAssistantDeviceComponent::new_switch(
                SWITCH_COMPONENT,
                "test_switch_command_topic/set",
                "test_component_unique_id")
        );

        check_serialization(
&HomeAssistantDeviceConfiguration::new(
                "test_device_id".to_string(),
                "Test Device Name".to_string(),
                "Test Origin Name".to_string(),
                "1.2.3(test)".to_string(),
                "test_device/state".to_string(),
                cmps
            )
        );
    }
}