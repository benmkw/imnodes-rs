use imgui::{im_str, Slider, Ui};

use imnodes::{
    editor, AttributeFlag, AttributeId, Context, EditorContext, IdentifierGenerator, InputPinId,
    LinkId, NodeId, OutputPinId, PinShape,
};

pub struct State {
    editor_context: EditorContext,
    id_gen: IdentifierGenerator,

    nodes: Graph,
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

impl Graph {
    fn new(id_gen: &mut IdentifierGenerator) -> Self {
        Self {
            nodes: vec![Node {
                id: id_gen.next_node(),
                value: 0.0, // never used
                typ: NodeType::Output(OutData {
                    input_red: id_gen.next_input_pin(),
                    input_green: id_gen.next_input_pin(),
                    input_blue: id_gen.next_input_pin(),
                    red: 0.1,
                    green: 0.1,
                    blue: 0.1,
                }),
                updated: false,
            }],
            links: vec![],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Link {
    id: LinkId,
    start: OutputPinId,
    end: InputPinId,
}

#[derive(Debug, Clone)]
struct Node {
    id: NodeId,
    typ: NodeType,
    value: f32,
    // for cycle detection
    updated: bool,
}

impl Node {
    fn has_output(&self, out: OutputPinId) -> bool {
        match self.typ {
            NodeType::Add(AddData { output, .. })
            | NodeType::Multiply(MultData { output, .. })
            | NodeType::Sine(SineData { output, .. })
            | NodeType::Time(TimeData { output, .. })
            | NodeType::Constant(ValueData { output, .. }) => output == out,
            NodeType::Output(_) => false,
        }
    }
    fn get_inputs(&self) -> Vec<InputPinId> {
        match self.typ {
            NodeType::Add(AddData { input, .. })
            | NodeType::Multiply(MultData { input, .. })
            | NodeType::Sine(SineData { input, .. }) => vec![input],
            NodeType::Output(OutData {
                input_red,
                input_green,
                input_blue,
                ..
            }) => vec![input_red, input_green, input_blue],
            NodeType::Time(_) | NodeType::Constant(_) => vec![],
        }
    }
}

fn update(graph: &mut Graph, curr_node_idx: usize, input_pin: Option<InputPinId>) {
    let links = &graph.links;

    let curr_node = graph.nodes[curr_node_idx].clone();

    let predecessors = graph
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(i, input_node)| {
            let is_connected = |link: &Link| {
                if let Some(input_pin) = input_pin {
                    input_node.has_output(link.start) && input_pin == link.end
                } else {
                    input_node.has_output(link.start) && curr_node.get_inputs().contains(&link.end)
                }
            };

            match links.iter().any(|link| is_connected(link)) {
                true => Some(i),
                false => None,
            }
        })
        .collect::<Vec<_>>();

    for p in &predecessors {
        if !graph.nodes[*p].updated {
            graph.nodes[*p].updated = true;
            update(graph, *p, None);
        }
    }

    // TODO do is this the best way to do this?
    let nodes = &mut graph.nodes;
    // SAFETY because we have no cycles, `curr_node` is never accessed through `nodes`
    let curr_node = unsafe { &mut *((&mut nodes[curr_node_idx]) as *mut Node) };

    match curr_node.typ {
        NodeType::Add(_) => {
            curr_node.value = predecessors
                .iter()
                .fold(0.0, |acc, x| acc + nodes[*x].value)
        }
        NodeType::Multiply(_) => {
            curr_node.value = predecessors
                .iter()
                .fold(1.0, |acc, x| acc * nodes[*x].value)
        }
        NodeType::Output(OutData {
            ref mut red,
            ref mut green,
            ref mut blue,
            ref input_red,
            ref input_green,
            ref input_blue,
            ..
        }) => {
            let total_val = predecessors
                .iter()
                .fold(0.0, |acc, x| acc + nodes[*x].value);
            let input_pin = input_pin.unwrap();
            if input_pin == *input_red {
                *red = total_val;
            } else if input_pin == *input_green {
                *green = total_val;
            } else if input_pin == *input_blue {
                *blue = total_val;
            }
        }
        NodeType::Sine(_) => {
            curr_node.value = if let Some(input) = predecessors.get(0) {
                (nodes[*input].value * std::f32::consts::PI).sin()
            } else {
                0.0
            }
        }
        NodeType::Time(_) => {
            curr_node.value = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                % 1000) as f32
                / 1000.0;
        }
        NodeType::Constant(_) => {
            dbg!(&predecessors.iter().collect::<Vec<_>>());
            // nothing to do
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    Add(AddData),
    Multiply(MultData),
    Output(OutData),
    Sine(SineData),
    Time(TimeData),
    Constant(ValueData),
}

#[derive(Debug, Clone, PartialEq)]
struct AddData {
    input: InputPinId,
    output: OutputPinId,
}

#[derive(Debug, Clone, PartialEq)]
struct MultData {
    input: InputPinId,
    output: OutputPinId,
}
#[derive(Debug, Clone, PartialEq)]
struct OutData {
    input_red: InputPinId,
    input_green: InputPinId,
    input_blue: InputPinId,
    red: f32,
    green: f32,
    blue: f32,
}
#[derive(Debug, Clone, PartialEq)]
struct SineData {
    input: InputPinId,
    output: OutputPinId,
}
#[derive(Debug, Clone, PartialEq)]
struct TimeData {
    input: InputPinId,
    output: OutputPinId,
}
#[derive(Debug, Clone, PartialEq)]
struct ValueData {
    input: InputPinId,
    output: OutputPinId,
    attribute: AttributeId,
}

impl State {
    pub fn new(context: &Context) -> Self {
        let editor_context = context.create_editor();
        let mut id_gen = editor_context.new_identifier_generator();
        let nodes = Graph::new(&mut id_gen);

        Self {
            id_gen,
            editor_context,
            nodes,
        }
    }
}

/// https://github.com/Nelarius/imnodes/blob/master/example/color_node_editor.cpp
///
/// TODO
/// - add more mouse keyboard modifier
pub fn show(ui: &Ui, state: &mut State) {
    state.editor_context.set_style_colors_classic();

    let background = if let NodeType::Output(OutData {
        red, green, blue, ..
    }) = &state.nodes.nodes[0].typ
    {
        imnodes::ColorStyle::GridBackground.push_color([*red, *green, *blue], &state.editor_context)
    } else {
        unreachable!()
    };

    let popup_modal = im_str!("popup_add_node");

    if ui.button(im_str!("Add Node"), [0.0, 0.0]) {
        ui.open_popup(popup_modal);
    }

    ui.popup_modal(popup_modal).build(|| {
        if ui.button(im_str!("Add"), [0.0, 0.0]) {
            state.nodes.nodes.push(Node {
                id: state.id_gen.next_node(),
                value: 0.0,
                typ: NodeType::Add(AddData {
                    input: state.id_gen.next_input_pin(),
                    output: state.id_gen.next_output_pin(),
                }),
                updated: false,
            });

            ui.close_current_popup();
        }
        if ui.button(im_str!("Multiply"), [0.0, 0.0]) {
            state.nodes.nodes.push(Node {
                id: state.id_gen.next_node(),
                value: 0.0,
                typ: NodeType::Multiply(MultData {
                    input: state.id_gen.next_input_pin(),
                    output: state.id_gen.next_output_pin(),
                }),
                updated: false,
            });
            ui.close_current_popup();
        }
        if ui.button(im_str!("Sine"), [0.0, 0.0]) {
            state.nodes.nodes.push(Node {
                id: state.id_gen.next_node(),
                value: 0.0,
                typ: NodeType::Sine(SineData {
                    input: state.id_gen.next_input_pin(),
                    output: state.id_gen.next_output_pin(),
                }),
                updated: false,
            });
            ui.close_current_popup();
        }
        if ui.button(im_str!("Time"), [0.0, 0.0]) {
            state.nodes.nodes.push(Node {
                id: state.id_gen.next_node(),
                value: 0.5,
                typ: NodeType::Time(TimeData {
                    input: state.id_gen.next_input_pin(),
                    output: state.id_gen.next_output_pin(),
                }),
                updated: false,
            });
            ui.close_current_popup();
        }
        if ui.button(im_str!("Constant"), [0.0, 0.0]) {
            state.nodes.nodes.push(Node {
                id: state.id_gen.next_node(),
                value: 0.0,
                typ: NodeType::Constant(ValueData {
                    input: state.id_gen.next_input_pin(),
                    output: state.id_gen.next_output_pin(),
                    attribute: state.id_gen.next_attribute(),
                }),
                updated: false,
            });
            ui.close_current_popup();
        }

        ui.separator();

        if ui.button(im_str!("Close"), [0.0, 0.0]) {
            ui.close_current_popup();
        }

        ui.separator();

        ui.text_wrapped(&im_str!("{:?}", &state.nodes));
    });

    let on_snap = state
        .editor_context
        .push(AttributeFlag::EnableLinkCreationOnSnap);
    let detach = state
        .editor_context
        .push(AttributeFlag::EnableLinkDetachWithDragClick);

    let State {
        ref mut editor_context,
        ref mut nodes,
        ..
    } = state;

    // propagate the values through the graph
    {
        let (input_red, input_green, input_blue) = if let NodeType::Output(OutData {
            input_red,
            input_green,
            input_blue,
            ..
        }) = nodes.nodes[0].typ
        {
            (input_red, input_green, input_blue)
        } else {
            unreachable!()
        };

        update(nodes, 0, Some(input_red));
        update(nodes, 0, Some(input_green));
        update(nodes, 0, Some(input_blue));
        for node in &mut nodes.nodes {
            node.updated = false;
        }
    }

    // main node ui
    let outer_scope = editor(editor_context, |mut editor| {
        for curr_node in nodes.nodes.iter_mut() {
            match curr_node.typ {
                NodeType::Add(AddData { input, output, .. }) => {
                    editor.add_node(curr_node.id, |mut node| {
                        node.add_titlebar(|| {
                            ui.text(im_str!("Add"));
                        });

                        node.add_input(input, PinShape::QuadFilled, || {
                            ui.text(im_str!("input"));
                        });

                        ui.text(im_str!("Value: {:.2}", curr_node.value));

                        node.add_output(output, PinShape::CircleFilled, || {
                            ui.text(im_str!("output"));
                        });
                    });
                }
                NodeType::Multiply(MultData { input, output, .. }) => {
                    editor.add_node(curr_node.id, |mut node| {
                        node.add_titlebar(|| {
                            ui.text(im_str!("Multiply"));
                        });

                        ui.text(im_str!("Value: {:.2}", curr_node.value));

                        node.add_input(input, PinShape::QuadFilled, || {
                            ui.text(im_str!("input"));
                        });

                        node.add_output(output, PinShape::CircleFilled, || {
                            ui.text(im_str!("output"));
                        });
                    });
                }
                NodeType::Output(OutData {
                    input_red,
                    input_green,
                    input_blue,
                    red,
                    green,
                    blue,
                    ..
                }) => {
                    editor.add_node(curr_node.id, |mut node| {
                        node.add_titlebar(|| {
                            ui.text(im_str!("Output"));
                        });

                        node.add_input(input_red, PinShape::QuadFilled, || {
                            ui.text(im_str!("red"));
                        });

                        node.add_input(input_green, PinShape::QuadFilled, || {
                            ui.text(im_str!("green"));
                        });

                        node.add_input(input_blue, PinShape::QuadFilled, || {
                            ui.text(im_str!("blue"));
                        });

                        ui.text(im_str!("red: {:.2}", red));
                        ui.text(im_str!("gree: {:.2}", green));
                        ui.text(im_str!("blue: {:.2}", blue));
                    });
                }
                NodeType::Sine(SineData { input, output, .. }) => {
                    editor.add_node(curr_node.id, |mut node| {
                        node.add_titlebar(|| {
                            ui.text(im_str!("Sine"));
                        });

                        node.add_input(input, PinShape::QuadFilled, || {
                            ui.text(im_str!("input"));
                        });

                        // TODO add modal for things other than sine?
                        ui.text(im_str!("Value: {:.2}", curr_node.value));

                        node.add_output(output, PinShape::CircleFilled, || {
                            ui.text(im_str!("output"));
                        });
                    });
                }
                NodeType::Time(TimeData { output, .. }) => {
                    editor.add_node(curr_node.id, |mut node| {
                        node.add_titlebar(|| {
                            ui.text(im_str!("Time"));
                        });

                        ui.text(im_str!("Value: {:.2}", curr_node.value));

                        node.add_output(output, PinShape::CircleFilled, || {
                            ui.text(im_str!("output"));
                        });
                    });
                }
                NodeType::Constant(ValueData {
                    attribute, output, ..
                }) => {
                    editor.add_node(curr_node.id, |mut node| {
                        node.add_titlebar(|| {
                            ui.text(im_str!("Constant"));
                        });

                        node.attribute(attribute, || {
                            ui.set_next_item_width(130.0);
                            Slider::new(im_str!("value"))
                                .range(0.0..=1.0)
                                .display_format(&im_str!("{:.2}", curr_node.value))
                                .build(&ui, &mut curr_node.value);
                        });

                        node.add_output(output, PinShape::CircleFilled, || {
                            ui.text(im_str!("output"));
                        });
                    });
                }
            }
        }

        for Link { id, start, end } in &nodes.links {
            editor.add_link(*id, *end, *start);
        }
    });

    if let Some(link) = outer_scope.links_created() {
        state.nodes.links.push(Link {
            id: state.id_gen.next_link(),
            start: link.start_pin,
            end: link.end_pin,
        })
    }

    if let Some(link) = outer_scope.get_dropped_link() {
        state
            .nodes
            .links
            .swap_remove(state.nodes.links.iter().position(|e| e.id == link).unwrap());
    }

    background.pop();
    on_snap.pop();
    detach.pop();
}
