use pumpkin_plugin_api::Context;

mod server_listener;

pub fn init(context: &Context) -> pumpkin_plugin_api::Result<()> {
    server_listener::start(&context)?;
    Ok(())
}