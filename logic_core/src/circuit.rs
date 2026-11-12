use std::collections::BTreeMap;

use crate::{InputPinId, NandGate, OutputPinId, Signal, WireError, validate_wire};

use super::{GateId, Wire};

pub struct Gate {
    id: GateId,
    inputs: [Signal; 2],
    output: Signal,
}

impl Gate {
    fn new(id: GateId, inputs: [Signal; 2]) -> Self {
        Self {
            id,
            inputs,
            output: NandGate::eval(inputs[0], inputs[1]),
        }
    }

    pub fn id(&self) -> GateId {
        self.id
    }

    pub fn input_values(&self) -> [Signal; 2] {
        self.inputs
    }

    pub fn output_value(&self) -> Signal {
        self.output
    }
}

pub struct Circuit {
    next_gate_id: GateId,
    gates: BTreeMap<GateId, Gate>,
    wires: Vec<Wire>,
    inputs: Vec<GateId>,
    outputs: Vec<GateId>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            next_gate_id: GateId(0),
            gates: BTreeMap::new(),
            wires: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_gate(&mut self) -> GateId {
        let id = self.next_gate_id;
        let new_gate = Gate::new(id, [false, false]);
        self.gates.insert(id, new_gate);
        self.next_gate_id.0 += 1;
        return id;
    }

    pub fn add_wire(&mut self, from: OutputPinId, to: InputPinId) -> Result<(), AddWireError> {
        if self.gates.get(&from.gate).is_none() {
            return Err(AddWireError::MissingGate(from.gate));
        }
        if self.gates.get(&to.gate).is_none() {
            return Err(AddWireError::MissingGate(to.gate));
        }
        if from.index >= NandGate::OUTPUT_COUNT {
            return Err(AddWireError::OutputIndexOutOfRange {
                gate: from.gate,
                index: from.index,
            });
        }
        if to.index >= NandGate::INPUT_COUNT {
            return Err(AddWireError::InputIndexOutOfRange {
                gate: to.gate,
                index: to.index,
            });
        }
        let wire = Wire { from, to };
        if let Err(e) = validate_wire(&wire, &self.wires) {
            return Err(AddWireError::Wire(e));
        }
        self.wires.push(wire);

        Ok(())
    }

    pub fn gate(&self, id: GateId) -> Option<&Gate> {
        self.gates.get(&id)
    }

    pub fn gate_mut(&mut self, id: GateId) -> Option<&mut Gate> {
        self.gates.get_mut(&id)
    }

    pub fn gates(&self) -> impl Iterator<Item = &Gate> {
        self.gates.values()
    }

    pub fn wires(&self) -> &[Wire] {
        &self.wires
    }

    pub fn inputs(&self) -> &[GateId] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[GateId] {
        &self.outputs
    }
}

pub enum AddWireError {
    MissingGate(GateId),
    OutputIndexOutOfRange { gate: GateId, index: u8 },
    InputIndexOutOfRange { gate: GateId, index: u8 },
    Wire(WireError),
}
