use super::{InputId, NandSimulation};

pub enum Command {
    ToggleInput(InputId),
}

impl Command {
    fn apply(&self, sim: &mut NandSimulation) {
        match self {
            Command::ToggleInput(id) => {
                sim.toggle(*id);
            }
        }
    }
    fn undo(&self, sim: &mut NandSimulation) {
        match self {
            Command::ToggleInput(id) => {
                sim.toggle(*id);
            }
        }
    }
}

// TODO: ADD a limit on how big the history can be.
#[derive(Default)]
pub struct CommandHistory {
    past: Vec<Command>,
    future: Vec<Command>,
}

impl CommandHistory {
    pub fn push(&mut self, cmd: Command, sim: &mut NandSimulation) {
        self.future.clear();
        cmd.apply(sim);
        self.past.push(cmd);
    }
    pub fn undo(&mut self, sim: &mut NandSimulation) -> bool {
        if let Some(cmd) = self.past.pop() {
            cmd.undo(sim);
            self.future.push(cmd);
            return true;
        }
        false
    }
    pub fn redo(&mut self, sim: &mut NandSimulation) -> bool {
        if let Some(cmd) = self.future.pop() {
            cmd.apply(sim);
            self.past.push(cmd);
            return true;
        }
        false
    }
}

// TODO: ADD a limit on how big the history can be.
#[derive(Default)]
pub struct CommandPending(pub Vec<Command>);
