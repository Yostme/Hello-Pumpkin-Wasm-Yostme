use pumpkin_plugin_api::Context;

mod roll_command;

pub fn init(context: &Context) -> pumpkin_plugin_api::Result<()> {
    // roll command
    roll_command::build(&context, &["roll".to_string()], "Rolls a die.")?;
    Ok(())
}
