use imnodes::{
    AttributeFlags, AttributeId, Context, EditorContext, IdentifierGenerator, InputPinId, LinkId,
    NodeId, OutputPinId, PinShape, Style, editor,
};

pub struct State {
    pub editor_context: EditorContext,
    id_gen: IdentifierGenerator,
    graph: Graph,
    style: Style,
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

impl Graph {
    fn new(id_gen: &mut IdentifierGenerator) -> Self {
        let output = id_gen.next_node();
        let red = id_gen.next_input_pin();
        let constant = id_gen.next_output_pin();

        Self {
            nodes: vec![
                Node {
                    id: output,
                    value: 0.0, // never used
                    typ: NodeType::Output(OutData {
                        input_red: red,
                        input_green: id_gen.next_input_pin(),
                        input_blue: id_gen.next_input_pin(),
                        red: 0.1,
                        green: 0.1,
                        blue: 0.1,
                    }),
                    updated: false,
                },
                Node {
                    id: id_gen.next_node(),
                    typ: NodeType::Constant(ConstData {
                        output: constant,
                        attribute: id_gen.next_attribute(),
                    }),
                    value: 0.4,
                    updated: false,
                },
            ],
            links: vec![Link {
                id: id_gen.next_link(),
                start: constant,
                end: red,
            }],
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
            | NodeType::Constant(ConstData { output, .. }) => output == out,
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

// Helper to get predecessor values without causing borrow issues
fn get_predecessor_values(graph: &Graph, predecessors: &[usize]) -> Vec<f32> {
    predecessors
        .iter()
        .filter_map(|&idx| graph.nodes.get(idx).map(|node| node.value))
        .collect()
}

// Recursive update function - separated logic to avoid borrow issues
fn update_node_recursive(graph: &mut Graph, node_idx: usize) {
    // Avoid re-updating if already processed in this pass
    if graph.nodes.get(node_idx).is_none_or(|n| n.updated) {
        return;
    }

    let node_clone = match graph.nodes.get(node_idx) {
        Some(n) => n.clone(),
        // Node index out of bounds
        None => return,
    };

    // Find direct predecessors based on links targeting this node's inputs
    let predecessors: Vec<usize> = graph
        .links
        .iter()
        .filter(|link| node_clone.get_inputs().contains(&link.end)) // Link ends at one of our inputs
        .filter_map(|link| {
            // Find the node which has the output pin where the link starts
            graph
                .nodes
                .iter()
                .position(|node| node.has_output(link.start))
        })
        .collect();

    // Recursively update predecessors first
    for &p_idx in &predecessors {
        update_node_recursive(graph, p_idx);
    }

    // Now that predecessors are updated, calculate this node's value
    let predecessor_values = get_predecessor_values(graph, &predecessors);

    // Get mutable access to the current node *after* processing predecessors
    if let Some(curr_node) = graph.nodes.get_mut(node_idx) {
        match curr_node.typ {
            NodeType::Add(_) => {
                curr_node.value = predecessor_values.iter().sum();
            }
            NodeType::Multiply(_) => {
                curr_node.value = predecessor_values.iter().product();
            }
            NodeType::Sine(_) => {
                // Sine usually takes one input
                curr_node.value = predecessor_values
                    .first()
                    .map_or(0.0, |&v| (v * core::f32::consts::PI).sin());
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
                // Value is updated by UI, nothing to do here
            }
            NodeType::Output(_) => {
                // Output node value itself isn't calculated, its pin values are.
                // This case should ideally not be reached via this recursive update path
                // unless it's the target node itself.
            }
        }
        // Mark as updated for this pass
        curr_node.updated = true;
    }
}

// Update function specifically for the output node pins
fn update_output_node(graph: &mut Graph, output_node_idx: usize) {
    // Reset all update flags before processing the output node
    for node in &mut graph.nodes {
        node.updated = false;
    }

    let output_node_data = match graph.nodes.get(output_node_idx) {
        Some(Node {
            typ: NodeType::Output(data),
            ..
        }) => data.clone(),
        _ => return, // Not the output node or index out of bounds
    };

    let pins_to_update = [
        output_node_data.input_red,
        output_node_data.input_green,
        output_node_data.input_blue,
    ];

    let mut final_values = [0.0f32; 3]; // R, G, B

    for (i, &pin_id) in pins_to_update.iter().enumerate() {
        // Find the single node connected to this specific input pin
        let source_node_idx = graph
            .links
            .iter()
            .filter(|link| link.end == pin_id)
            .find_map(|link| {
                graph
                    .nodes
                    .iter()
                    .position(|node| node.has_output(link.start))
            });

        if let Some(idx) = source_node_idx {
            // Reset flags before updating this specific path
            for node in &mut graph.nodes {
                node.updated = false;
            }
            // Update the subgraph leading to this source node
            update_node_recursive(graph, idx);
            // Get the final value from the source node
            final_values[i] = graph.nodes.get(idx).map_or(0.0, |n| n.value);
        } else {
            final_values[i] = 0.1; // Default if no node connected
        }
    }

    // Apply the calculated values to the output node
    if let Some(Node {
        typ: NodeType::Output(data),
        ..
    }) = graph.nodes.get_mut(output_node_idx)
    {
        data.red = final_values[0];
        data.green = final_values[1];
        data.blue = final_values[2];
    }
}

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    Add(AddData),
    Multiply(MultData),
    Output(OutData),
    Sine(SineData),
    Time(TimeData),
    Constant(ConstData),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AddData {
    input: InputPinId,
    output: OutputPinId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
struct SineData {
    input: InputPinId,
    output: OutputPinId,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct TimeData {
    output: OutputPinId,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct ConstData {
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
            graph: nodes,
            style: Style::default(),
        }
    }
}

/// https://github.com/Nelarius/imnodes/blob/master/example/color_node_editor.cpp
pub fn show(ui: &imgui::Ui, state: &mut State) {
    state
        .editor_context
        .set_style_colors_classic(&mut state.style);

    ui.text("press \"A\" or right click to add a Node");

    // Update graph values before getting colors
    update_output_node(&mut state.graph, 0); // Assuming node 0 is always the output node

    // color setup
    let background = if let NodeType::Output(OutData {
        red, green, blue, ..
    }) = &state.graph.nodes[0].typ
    {
        imnodes::ColorStyle::GridBackground.push_color([*red, *green, *blue], &state.editor_context)
    } else {
        unreachable!()
    };

    let title_bar_color = imnodes::ColorStyle::TitleBar.push_color(
        [11.0 / 255.0, 109.0 / 255.0, 191.0 / 255.0],
        &state.editor_context,
    );
    let title_bar_hovered_color = imnodes::ColorStyle::TitleBarHovered.push_color(
        [45.0 / 255.0, 126.0 / 255.0, 194.0 / 255.0],
        &state.editor_context,
    );
    let title_bar_selected_color = imnodes::ColorStyle::TitleBarSelected.push_color(
        [81.0 / 255.0, 148.0 / 255.0, 204.0 / 255.0],
        &state.editor_context,
    );

    let link_color = imnodes::ColorStyle::Link.push_color([0.8, 0.5, 0.1], &state.editor_context);

    // Set node position here, after the editor context is set
    let _ = state.editor_context.set_as_current_editor(); // Ensure context is current before setting pos
    let width = ui.window_content_region_max()[0] - ui.window_content_region_min()[0];
    let _ = state.graph.nodes[0]
        .id
        .set_position(0.9 * width, 300.0, imnodes::CoordinateSystem::ScreenSpace)
        .set_draggable(false);

    // node and link behaviour setup
    let on_snap = state
        .editor_context
        .push_attribute_flag(AttributeFlags::EnableLinkCreationOnSnap);
    let detach = state
        .editor_context
        .push_attribute_flag(AttributeFlags::EnableLinkDetachWithDragClick);

    // Destructure state *after* graph update
    let State {
        editor_context,
        graph,
        id_gen,
        ..
    } = state;

    // main node ui
    let outer_scope = create_the_editor(ui, editor_context, graph, id_gen);

    // user interaction handling
    if let Some(link) = outer_scope.links_created() {
        state.graph.links.push(Link {
            id: state.id_gen.next_link(),
            start: link.start_pin,
            end: link.end_pin,
        });
        // Trigger graph update after link creation
        update_output_node(&mut state.graph, 0);
    }

    if let Some(link) = outer_scope.get_destroyed_link() {
        state.graph.links.retain(|e| e.id != link);
        // Trigger graph update after link destruction
        update_output_node(&mut state.graph, 0);
    }

    // cleanup
    background.pop();

    title_bar_color.pop();
    title_bar_hovered_color.pop();
    title_bar_selected_color.pop();
    link_color.pop();

    on_snap.pop();
    detach.pop();
}

/// main node ui
fn create_the_editor(
    ui: &imgui::Ui,
    editor_context: &mut EditorContext,
    graph: &mut Graph,
    id_gen: &mut IdentifierGenerator,
) -> imnodes::OuterScope {
    editor(editor_context, |mut editor| {
        editor.add_mini_map(0.2, imnodes::MiniMapLocation::BottomLeft);

        let popup_modal = "popup_add_node";

        if editor.is_hovered()
            && (ui.is_mouse_clicked(imgui::MouseButton::Right) || ui.is_key_released(imgui::Key::A))
        {
            ui.open_popup(popup_modal);
        }

        ui.modal_popup_config(popup_modal)
            .always_auto_resize(true)
            .movable(false)
            .resizable(false)
            .build(|| {
                let size = [100.0, 0.0];
                // Get mouse position relative to screen space as that's where node positions are set
                let click_pos = ui.io().mouse_pos;

                // Helper closure to generate node ID and set position
                // Does NOT capture id_gen mutably anymore
                let gen_node_base = |id_gen: &mut IdentifierGenerator| {
                    let node_id = id_gen.next_node();
                    // Set position immediately after creation
                    let _ = node_id.set_position(
                        click_pos[0],
                        click_pos[1],
                        imnodes::CoordinateSystem::ScreenSpace,
                    );
                    node_id
                };

                if ui.button_with_size("Add", size) {
                    // Generate IDs *here*, outside the closure
                    let node_id = gen_node_base(id_gen);
                    let input_pin = id_gen.next_input_pin();
                    let output_pin = id_gen.next_output_pin();
                    graph.nodes.push(Node {
                        id: node_id,
                        value: 0.0,
                        typ: NodeType::Add(AddData {
                            input: input_pin,
                            output: output_pin,
                        }),
                        updated: false,
                    });
                    ui.close_current_popup();
                }
                if ui.button_with_size("Multiply", size) {
                    let node_id = gen_node_base(id_gen);
                    let input_pin = id_gen.next_input_pin();
                    let output_pin = id_gen.next_output_pin();
                    graph.nodes.push(Node {
                        id: node_id,
                        value: 0.0,
                        typ: NodeType::Multiply(MultData {
                            input: input_pin,
                            output: output_pin,
                        }),
                        updated: false,
                    });
                    ui.close_current_popup();
                }
                if ui.button_with_size("Sine", size) {
                    graph.nodes.push(Node {
                        id: gen_node_base(id_gen),
                        value: 0.0,
                        typ: NodeType::Sine(SineData {
                            input: id_gen.next_input_pin(),
                            output: id_gen.next_output_pin(),
                        }),
                        updated: false,
                    });
                    ui.close_current_popup();
                }
                if ui.button_with_size("Time", size) {
                    graph.nodes.push(Node {
                        id: gen_node_base(id_gen),
                        value: 0.0,
                        typ: NodeType::Time(TimeData {
                            output: id_gen.next_output_pin(),
                        }),
                        updated: false,
                    });
                    ui.close_current_popup();
                }
                if ui.button_with_size("Constant", size) {
                    let node_id = gen_node_base(id_gen);
                    let output_pin = id_gen.next_output_pin();
                    let attr_id = id_gen.next_attribute();
                    graph.nodes.push(Node {
                        id: node_id,
                        value: 0.0,
                        typ: NodeType::Constant(ConstData {
                            output: output_pin,
                            attribute: attr_id,
                        }),
                        updated: false,
                    });
                    ui.close_current_popup();
                }
                ui.separator();
                if ui.button_with_size("Close", size) {
                    ui.close_current_popup();
                }
            });

        for i in 0..graph.nodes.len() {
            let node_id = graph.nodes[i].id;
            let node_type = graph.nodes[i].typ.clone();
            let node_value = graph.nodes[i].value;

            editor.add_node(node_id, |mut node_scope| {
                match node_type {
                    NodeType::Add(AddData { input, output, .. }) => {
                        node_scope.add_titlebar(|| ui.text("Add"));
                        node_scope.add_input(input, PinShape::QuadFilled, || ui.text("input"));
                        ui.text(format!("Value: {node_value:.2}"));
                        node_scope.add_output(output, PinShape::CircleFilled, || ui.text("output"));
                    }
                    NodeType::Multiply(MultData { input, output, .. }) => {
                        node_scope.add_titlebar(|| ui.text("Multiply"));
                        ui.text(format!("Value: {node_value:.2}"));
                        node_scope.add_input(input, PinShape::QuadFilled, || ui.text("input"));
                        node_scope.add_output(output, PinShape::CircleFilled, || ui.text("output"));
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
                        node_scope.add_titlebar(|| ui.text("Output"));
                        node_scope.add_input(input_red, PinShape::QuadFilled, || ui.text("red"));
                        node_scope
                            .add_input(input_green, PinShape::QuadFilled, || ui.text("green"));
                        node_scope.add_input(input_blue, PinShape::QuadFilled, || ui.text("blue"));
                        // Read directly from the cloned data for display
                        ui.text(format!("red: {red:.2}"));
                        ui.text(format!("green: {green:.2}"));
                        ui.text(format!("blue: {blue:.2}"));
                    }
                    NodeType::Sine(SineData { input, output, .. }) => {
                        node_scope.add_titlebar(|| ui.text("Sine"));
                        node_scope.add_input(input, PinShape::QuadFilled, || ui.text("input"));
                        ui.text(format!("Value: {node_value:.2}"));
                        node_scope.add_output(output, PinShape::CircleFilled, || ui.text("output"));
                    }
                    NodeType::Time(TimeData { output, .. }) => {
                        node_scope.add_titlebar(|| ui.text("Time"));
                        ui.text(format!("Value: {node_value:.2}"));
                        node_scope.add_output(output, PinShape::CircleFilled, || ui.text("output"));
                    }
                    NodeType::Constant(ConstData {
                        attribute, output, ..
                    }) => {
                        node_scope.add_titlebar(|| ui.text("Constant"));
                        // Need mutable access *only* for the slider
                        node_scope.add_static_attribute(attribute, || {
                            ui.set_next_item_width(130.0);
                            // Get mutable access just for the slider build call
                            if let Some(node_mut) = graph.nodes.get_mut(i) {
                                if ui
                                    .slider_config("value", 0.0, 1.0)
                                    .display_format(format!("{:.2}", node_mut.value)) // Display current value
                                    .build(&mut node_mut.value)
                                // Modify actual value
                                {
                                    // Trigger graph update if constant value changes
                                    update_output_node(graph, 0);
                                }
                            }
                        });
                        node_scope.add_output(output, PinShape::CircleFilled, || ui.text("output"));
                    }
                } // end match
            }); // end add_node
        } // end for

        for Link { id, start, end } in &graph.links {
            editor.add_link(*id, *end, *start);
        }
    })
}
