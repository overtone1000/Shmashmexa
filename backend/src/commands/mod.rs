use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct ChangeDashData {index:u32}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum Command
{
    ChangeDash(ChangeDashData)
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
        check_serialization(&Command::ChangeDash(ChangeDashData { index: 3 }));
    }
}