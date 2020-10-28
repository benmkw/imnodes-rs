/*!
this is what I want to write but can't:
https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=4e4b74932e4ed7f0c097e10160df3384

this code is not pretty

PR welcome :)
*/

#![allow(missing_docs)]

use crate::InputPinId;

pub trait Graph {
    type Node: Clone;
    // (input pin id, index in nodes list)
    fn get_predecessor_node_indizes_of(&self, input_pin: (InputPinId, usize)) -> Vec<usize>;
    fn get_inputs_of_node_at(&self, index: usize) -> Vec<InputPinId>;
    fn get_node_mut(&mut self, index: usize) -> &mut Self::Node;
    fn clone_nodes(&self) -> Vec<Self::Node>;
}

fn recurse_on_postorder<G: Graph>(
    node: &G,
    input_pin: (InputPinId, usize),
    mut stack: &mut Vec<((InputPinId, usize), Vec<usize>)>,
) {
    let predecessors = Graph::get_predecessor_node_indizes_of(node, input_pin);
    for predecessor_index in &predecessors {
        if stack
            .iter()
            .find(|(inserted_node, _)| *predecessor_index == inserted_node.1)
            .is_none()
        {
            for input in Graph::get_inputs_of_node_at(node, *predecessor_index) {
                recurse_on_postorder(node, (input, *predecessor_index), &mut stack);
            }
        }
    }
    stack.push((input_pin, predecessors)); // postorder
}

// TODO test
fn recurse_on_preorder<G: Graph>(
    node: &G,
    input_pin: (InputPinId, usize),
    mut stack: &mut Vec<((InputPinId, usize), Vec<usize>)>,
) {
    let predecessors = Graph::get_predecessor_node_indizes_of(node, input_pin);
    stack.push((input_pin, predecessors.clone())); // preorder
    for predecessor_index in &predecessors {
        if stack
            .iter()
            .find(|(inserted_node, _)| *predecessor_index == inserted_node.1)
            .is_none()
        {
            for input in Graph::get_inputs_of_node_at(node, *predecessor_index) {
                recurse_on_postorder(node, (input, *predecessor_index), &mut stack);
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Order {
    Preorder,
    Postorder,
}

pub fn apply_fn<F: Fn(&mut <G as Graph>::Node, &[<G as Graph>::Node]), G: Graph>(
    graph: &mut G,
    start_pin: (InputPinId, usize),
    order: Order,
    f: F,
) {
    let mut indices = vec![];

    match order {
        Order::Postorder => {
            recurse_on_postorder(graph, start_pin, &mut indices);
        }
        Order::Preorder => {
            recurse_on_preorder(graph, start_pin, &mut indices);
        }
    }

    for (i, predeccessor_indices) in &indices {
        let predecessors = graph
            .clone_nodes()
            .iter()
            .enumerate()
            .filter_map(|(i, node)| {
                if predeccessor_indices.contains(&i) {
                    Some(node.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        f(graph.get_node_mut(i.1), &predecessors);
    }
}
