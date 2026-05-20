use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ChangeDashData {url:String}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Command
{
    AutoTab(String),
    SetScreenState(bool)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn check_serialization(command: &Command) {
        println!("Serialization test:");
        let serialized = serde_json::to_string(command).expect("Should serialize.");
        println!("   {}", serialized);
        let deserialized: Command = serde_json::from_str(&serialized).expect("Should deserialize.");
        println!("   {:?}", deserialized);
        assert_eq!(*command,deserialized)        
    }

    #[test]
    fn serialization() {
        check_serialization(&Command::AutoTab( "https://www.example.com".to_string()));
        check_serialization(&Command::SetScreenState(true));
    }
}