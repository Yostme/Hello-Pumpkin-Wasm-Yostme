use pumpkin_plugin_api::{command::{CommandError, CommandSender, ConsumedArgs, Command, CommandNode}, commands::CommandHandler, permission::{Permission, PermissionDefault}, text::TextComponent, Context, Server};
use rand::{rng, RngExt};

pub fn build(context: &Context, names: &[String], description: &str) -> pumpkin_plugin_api::Result<()> {
    context.register_permission(&Permission {
        node: "essentials:command.roll".to_string(),
        description: "Allows the player to roll a die.".to_string(),
        default: PermissionDefault::Allow,
        children: Vec::new(),
    })?;

    context.register_command(Command::new(
        &names,
        description,
    ) // Look into using an argument instead! <---------------------------------
        .then(CommandNode::literal("D2")
            .execute(RollExecutor(Die::D2)))
        .then(CommandNode::literal("D4")
            .execute(RollExecutor(Die::D4)))
        .then(CommandNode::literal("D6")
            .execute(RollExecutor(Die::D6)))
        .then(CommandNode::literal("D8")
            .execute(RollExecutor(Die::D8)))
        .then(CommandNode::literal("D10")
            .execute(RollExecutor(Die::D10)))
        .then(CommandNode::literal("D12")
            .execute(RollExecutor(Die::D12)))
        .then(CommandNode::literal("D20")
            .execute(RollExecutor(Die::D20))
        ), "essentials:command.roll");
    Ok(())
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Die {
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl Die {
    fn roll_die(&self) -> i32 {
        match self {
            Die::D2 => {
                let roll = rng().random_range(1..=2);
                roll
            }
            Die::D4 => {
                let roll = rng().random_range(1..=4);
                roll
            }
            Die::D6 => {
                let roll = rng().random_range(1..=6);
                roll
            }
            Die::D8 => {
                let roll = rng().random_range(1..=8);
                roll
            }
            Die::D10 => {
                let roll = rng().random_range(1..=10);
                roll
            }
            Die::D12 => {
                let roll = rng().random_range(1..=12);
                roll
            }
            Die::D20 => {
                let roll = rng().random_range(1..=20);
                roll
            }
        }
    }
}

struct RollExecutor(pub Die);
impl CommandHandler for RollExecutor {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let roll = Die::roll_die(&self.0);
        let message = TextComponent::text(&format!("Rolling {:?}:", &self.0))
            .add_child(TextComponent::text(&format!("\nYou got a {:?}!", roll)));
        sender.send_message(message);
        Ok(1)
    }
}
