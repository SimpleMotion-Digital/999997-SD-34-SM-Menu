pub mod axis;
pub mod base;
pub mod edit;
pub mod file;
pub mod help;
pub mod load;
pub mod quit;
pub mod save;
pub mod show;
pub mod vers;
pub mod view;

use self::base::InfoCommand;
use crate::{CliResult, Command, CommandResult};

/// Root command that provides the main menu
#[derive(Debug)]
pub struct RootCommand;

impl Command for RootCommand {
    fn name(&self) -> &'static str {
        "root"
    }

    fn description(&self) -> &'static str {
        "simple-menu main menu with File, Edit, View, Help, and Quit commands"
    }

    fn execute(&mut self, _args: &[String]) -> CliResult<CommandResult> {
        Ok(CommandResult::Continue)
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(file::FileCommand::new()),
            Box::new(edit::EditCommand::new()),
            Box::new(view::ViewCommand::new()),
            Box::new(help::HelpCommand::new()),
            Box::new(quit::QuitCommand::new()),
            Box::new(InfoCommand::new(self.name())),
        ]
    }
}
