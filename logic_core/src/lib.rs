type Signal = bool;
type InputId = usize;

struct NandGate;

impl NandGate {
    fn eval(a: Signal, b: Signal) -> Signal {
        !(a && b)
    }
}

pub struct NandSimulation {
    inputs: [Signal; 2],
    outputs: [Signal; 1],
}

impl Default for NandSimulation {
    fn default() -> Self {
        Self {
            inputs: [false, false],
            outputs: [true],
        }
    }
}

impl NandSimulation {
    pub fn tick(&mut self) {
        self.outputs[0] = NandGate::eval(self.inputs[0], self.inputs[1])
    }

    // TODO: Add propper error propagation when an id does not exist
    fn toggle(&mut self, id: InputId) {
        if id > self.inputs.len() - 1 {
            panic!("INVALID INPUT ID... BIGGER THAN AVAILABLE INPUTS")
        }
        self.inputs[id] = !self.inputs[id]
    }

    pub fn get_out(&self) -> &[Signal] {
        &self.outputs
    }
}

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
