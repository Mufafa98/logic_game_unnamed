use std::{
    cmp::min,
    collections::{BTreeMap, BTreeSet},
};

use crate::{GateId, circuit::Circuit};

fn build_adjacency(circuit: &Circuit) -> BTreeMap<GateId, Vec<GateId>> {
    let mut successors = BTreeMap::new();

    for gate in circuit.gates() {
        let id = gate.id();
        successors.insert(id, Vec::new());
    }

    for wire in circuit.wires() {
        successors
            .entry(wire.from.gate)
            .or_default()
            .push(wire.to.gate);
    }

    for values in successors.values_mut() {
        values.sort_unstable();
    }

    successors
}

fn tarjan_scc(circuit: &Circuit) -> Vec<Vec<GateId>> {
    let adj = build_adjacency(circuit);
    let mut sccs = Vec::new();
    let mut index: BTreeMap<GateId, u32> = BTreeMap::new();
    let mut low_link: BTreeMap<GateId, u32> = BTreeMap::new();
    let mut on_stack: BTreeSet<GateId> = BTreeSet::new();
    let mut scc_stack = Vec::new();
    let mut counter = 0;

    for root_gate in circuit.gates() {
        let root_id = root_gate.id();
        if index.contains_key(&root_id) {
            continue;
        }

        let mut work = vec![(root_id, 0usize)];

        while let Some(&mut (node, ref mut next_child)) = work.last_mut() {
            if *next_child == 0 {
                index.insert(node, counter);
                low_link.insert(node, counter);
                counter += 1;
                scc_stack.push(node);
                on_stack.insert(node);
            }

            let successors = adj.get(&node).unwrap();

            if *next_child < successors.len() {
                let succ = successors[*next_child];
                *next_child += 1;

                if !index.contains_key(&succ) {
                    work.push((succ, 0));
                } else if on_stack.contains(&succ) {
                    let node_ll = low_link.get_mut(&node).unwrap();
                    *node_ll = min(*node_ll, index[&succ]);
                }
            } else {
                let (node, _) = work.pop().unwrap();

                if low_link[&node] == index[&node] {
                    let mut scc = vec![];
                    loop {
                        let succ = scc_stack.pop().unwrap();
                        on_stack.remove(&succ);
                        scc.push(succ);
                        if succ == node {
                            break;
                        }
                    }
                    scc.sort_unstable();
                    sccs.push(scc);
                }

                if let Some(&(parent, _)) = work.last() {
                    let node_ll = low_link[&node];
                    let parent_ll = low_link.get_mut(&parent).unwrap();
                    *parent_ll = min(*parent_ll, node_ll);
                }
            }
        }
    }

    sccs.reverse();
    sccs
}
