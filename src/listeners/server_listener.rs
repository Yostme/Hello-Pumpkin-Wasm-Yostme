use pumpkin_plugin_api::{
    events::{EventData, PlayerJoinEvent, PlayerLeaveEvent, EventPriority},
    text::{TextComponent},
    Context, EventHandler, Server,
};
use pumpkin_plugin_api::common::NamedColor;

pub fn start(context: &Context) -> pumpkin_plugin_api::Result<()> {
    context.register_event_handler(MyJoinHandler, EventPriority::Normal, true)?;
    context.register_event_handler(MyLeaveHandler, EventPriority::Normal, true)?;
    Ok(())
}

// Player Join / Leave listener
struct MyJoinHandler;
impl EventHandler<PlayerJoinEvent> for MyJoinHandler {
    fn handle<'a>(
        &'a self,
        _server: Server,
        mut event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        event.join_message = event.player.get_display_name()
            .add_child(TextComponent::text(" joined the server!"))
            .color_named(NamedColor::Yellow);
        event
    }
}

struct MyLeaveHandler;
impl EventHandler<PlayerLeaveEvent> for MyLeaveHandler {
    fn handle<'a>(
        &'a self,
        _server: Server,
        mut event: EventData<PlayerLeaveEvent>,
    ) -> EventData<PlayerLeaveEvent> {
        event.leave_message = event.player.get_display_name()
            .add_child(TextComponent::text(" left the server!"))
            .color_named(NamedColor::Yellow);
        event
    }
}
