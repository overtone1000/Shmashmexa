use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::commands::{self, Command};

#[derive(Clone)]
pub struct ExternalCore
{
    pub command_sender:UnboundedSender<Command>
}

impl ExternalCore
{
    pub fn new(command_sender:UnboundedSender<Command>)->ExternalCore
    {
        ExternalCore
        {
            command_sender
        }
    }


}