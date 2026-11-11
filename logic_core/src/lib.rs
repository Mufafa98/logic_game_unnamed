pub mod command;

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
