type Signal = bool;

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

impl NandSimulation {
    pub fn tick(&mut self) {
        self.outputs[0] = NandGate::eval(self.inputs[0], self.inputs[1])
    }
}
