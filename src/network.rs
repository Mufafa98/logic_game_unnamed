use std::collections::HashMap;

use crate::{
    app::{CompoundDef, Graph},
    consts::NETWORK_MAX_ITER,
    logic_item::{ItemKind, LogicItem},
    ports::PortKind,
};

pub struct Network {
    outputs: HashMap<u64, Vec<bool>>,
    inputs: HashMap<(u64, u8), bool>,
}

impl Network {
    pub fn evaluate(
        graph: &Graph,
        defs: &HashMap<u64, CompoundDef>,
        source_overrides: &HashMap<u64, bool>,
    ) -> Self {
        let items = &graph.items;
        let connections = &graph.connections;
        let idx_by_id: HashMap<u64, usize> = items
            .iter()
            .enumerate()
            .map(|(idx, item)| (item.id, idx))
            .collect();

        let mut item_inputs: Vec<Vec<Vec<(u64, u8)>>> = items
            .iter()
            .map(|item| vec![Vec::new(); item.no_in as usize])
            .collect();

        let mut outputs: Vec<Vec<bool>> = items
            .iter()
            .map(|iter| vec![false; iter.no_out as usize])
            .collect();

        for conn in connections {
            if let Some(&target_item_index) = idx_by_id.get(&conn.to.item_id)
                && let PortKind::Input(in_port) = conn.to.kind
                && let PortKind::Output(out_port) = conn.from.kind
            {
                let source_id = conn.from.item_id;
                item_inputs[target_item_index][in_port as usize].push((source_id, out_port));
            }
        }

        for _ in 0..NETWORK_MAX_ITER {
            let mut changed = false;

            for (idx, item) in items.iter().enumerate() {
                let inputs: Vec<bool> = item_inputs[idx]
                    .iter()
                    .map(|sources| {
                        sources.iter().any(|&(source_id, out_port)| {
                            output_value(&idx_by_id, &outputs, source_id, out_port)
                        })
                    })
                    .collect();
                let new = eval_item(item, &inputs, defs, source_overrides);
                if new != outputs[idx] {
                    outputs[idx] = new;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        let outputs = items
            .iter()
            .zip(outputs)
            .map(|(item, output)| (item.id, output))
            .collect();

        let mut network = Self {
            outputs,
            inputs: HashMap::new(),
        };

        for (idx, item) in items.iter().enumerate() {
            for (port_idx, sources) in item_inputs[idx].iter().enumerate() {
                let val = sources
                    .iter()
                    .any(|&(id, out_idx)| network.output(id, out_idx));
                network.inputs.insert((item.id, port_idx as u8), val);
            }
        }

        network
    }
    pub fn output(&self, item_id: u64, idx: u8) -> bool {
        self.outputs
            .get(&item_id)
            .and_then(|v| v.get(idx as usize))
            .copied()
            .unwrap_or(false)
    }

    pub fn input(&self, item_id: u64, idx: u8) -> bool {
        self.inputs.get(&(item_id, idx)).copied().unwrap_or(false)
    }
}
fn eval_item(
    item: &LogicItem,
    inputs: &[bool],
    defs: &HashMap<u64, CompoundDef>,
    overrides: &HashMap<u64, bool>,
) -> Vec<bool> {
    match item.kind {
        ItemKind::Source => {
            vec![overrides.get(&item.id).copied().unwrap_or(item.on)]
        }
        ItemKind::Led => Vec::new(),
        ItemKind::Gate(g) => g.eval(inputs),
        ItemKind::Compound(def_id) => match defs.get(&def_id) {
            Some(def) => eval_compound(def, inputs, defs),
            None => vec![false; item.no_out as usize],
        },
    }
}
fn eval_compound(
    def: &CompoundDef,
    inputs: &[bool],
    defs: &HashMap<u64, CompoundDef>,
) -> Vec<bool> {
    let mut overrides = HashMap::new();
    for (i, &src_id) in def.input_sources.iter().enumerate() {
        overrides.insert(src_id, inputs.get(i).copied().unwrap_or(false));
    }

    let net = Network::evaluate(&def.graph, defs, &overrides);

    def.output_leds
        .iter()
        .map(|&led_id| net.input(led_id, 0))
        .collect()
}
fn output_value(by_id: &HashMap<u64, usize>, outputs: &[Vec<bool>], id: u64, idx: u8) -> bool {
    by_id
        .get(&id)
        .and_then(|&i| outputs[i].get(idx as usize))
        .copied()
        .unwrap_or(false)
}
