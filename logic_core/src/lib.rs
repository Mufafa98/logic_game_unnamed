mod circuit;
pub mod command;
mod topo;

type Signal = bool;
type InputId = usize;

struct NandGate;

impl NandGate {
    pub const INPUT_COUNT: u8 = 2;
    pub const OUTPUT_COUNT: u8 = 1;
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GateId(pub u64);

#[derive(Clone, Copy, PartialEq)]
pub struct InputPinId {
    pub gate: GateId,
    pub index: u8,
}

pub struct OutputPinId {
    pub gate: GateId,
    pub index: u8,
}

pub struct Wire {
    pub from: OutputPinId,
    pub to: InputPinId,
}

pub enum WireError {
    DuplicateDestination(InputPinId),
}

pub fn validate_wire(wire: &Wire, existing: &[Wire]) -> Result<(), WireError> {
    if existing.iter().any(|w| w.to == wire.to) {
        return Err(WireError::DuplicateDestination(wire.to));
    }
    Ok(())
}
