use crate::commands::base::{SubBaseCommands};
use crate::eval::new_eval;
pub fn sub_base_eval (commands: SubBaseCommands, command_type: bool) {

    match commands {
       SubBaseCommands::New { commands } => new_eval::sub_new_eval(commands, command_type),
       SubBaseCommands::Update => {}
    }

}
