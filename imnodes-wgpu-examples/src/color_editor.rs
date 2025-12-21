use imnodes::{
    AttributeFlags, AttributeId, Context, EditorContext, EditorContextDesc, IdentifierGenerator,
    InputAttributeDesc, InputPinId, LinkId, MiniMapDesc, MiniMapLocation, NodeId,
    OutputAttributeDesc, OutputPinId, PinShape, StaticAttributeDesc, StyleDesc, editor,
};

pub struct State<'a> {
    pub editor_context: EditorContext<'a>,
    id_gen: IdentifierGenerator,
    graph: Graph,
    positions_initialized: bool,
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

impl Graph {
    fn new(id_gen: &mut IdentifierGenerator) -> Self {
        // Create output node
        let output = id_gen.next_node();
        let red_in = id_gen.next_input_pin();
        let green_in = id_gen.next_input_pin();
        let blue_in = id_gen.next_input_pin();

        // Create time node (oscillates)
        let time_node = id_gen.next_node();
        let time_out = id_gen.next_output_pin();

        // Create sine node (smooths the oscillation)
        let sine_node = id_gen.next_node();
        let sine_in = id_gen.next_input_pin();
        let sine_out = id_gen.next_output_pin();

        // Create multiply node (scales the sine wave)
        let multiply_node = id_gen.next_node();
        let multiply_in1 = id_gen.next_input_pin();
        let multiply_in2 = id_gen.next_input_pin();
        let multiply_out = id_gen.next_output_pin();

        // Create constant node for scaling
        let constant_node = id_gen.next_node();
        let constant_out = id_gen.next_output_pin();

        // Create add node to offset the result
        let add_node = id_gen.next_node();
        let add_in1 = id_gen.next_input_pin();
        let add_in2 = id_gen.next_input_pin();
        let add_out = id_gen.next_output_pin();

        let nodes = vec![
            Node {
                id: output,
                value: 0.0,
                typ: NodeType::Output(OutData {
                    input_red: red_in,
                    input_green: green_in,
                    input_blue: blue_in,
                    red: 0.1,
                    green: 0.1,
                    blue: 0.1,
                }),
                updated: false,
            },
            Node {
                id: time_node,
                typ: NodeType::Time(TimeData { output: time_out }),
                value: 0.0,
                updated: false,
            },
            Node {
                id: sine_node,
                typ: NodeType::Sine(SineData {
                    input: sine_in,
                    output: sine_out,
                }),
                value: 0.0,
                updated: false,
            },
            Node {
                id: multiply_node,
                typ: NodeType::Multiply(MultData {
                    input1: multiply_in1,
                    input2: multiply_in2,
                    output: multiply_out,
                }),
                value: 0.0,
                updated: false,
            },
            Node {
                id: constant_node,
                typ: NodeType::Constant(ConstData {
                    output: constant_out,
                    attribute: id_gen.next_attribute(),
                }),
                value: 0.5,
                updated: false,
            },
            Node {
                id: add_node,
                typ: NodeType::Add(AddData {
                    input1: add_in1,
                    input2: add_in2,
                    output: add_out,
                }),
                value: 0.0,
                updated: false,
            },
        ];

        // Create links to form a processing chain: time -> sine -> multiply -> add -> output (red channel)
        let links = vec![
            Link {
                id: id_gen.next_link(),
                start: time_out,
                end: sine_in,
            },
            Link {
                id: id_gen.next_link(),
                start: sine_out,
                end: multiply_in1,
            },
            Link {
                id: id_gen.next_link(),
                start: constant_out,
                end: multiply_in2,
            },
            Link {
                id: id_gen.next_link(),
                start: multiply_out,
                end: add_in1,
            },
            Link {
                id: id_gen.next_link(),
                start: constant_out,
                end: add_in2,
            },
            Link {
                id: id_gen.next_link(),
                start: add_out,
                end: red_in,
            },
        ];

        Self { nodes, links }
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

    fn owns_input(&self, pin_id: InputPinId) -> bool {
        match self.typ {
            NodeType::Add(AddData { input1, input2, .. }) => input1 == pin_id || input2 == pin_id,
            NodeType::Multiply(MultData { input1, input2, .. }) => {
                input1 == pin_id || input2 == pin_id
            }
            NodeType::Sine(SineData { input, .. }) => input == pin_id,
            NodeType::Output(OutData {
                input_red,
                input_green,
                input_blue,
                ..
            }) => input_red == pin_id || input_green == pin_id || input_blue == pin_id,
            NodeType::Time(_) | NodeType::Constant(_) => false,
        }
    }

    fn get_inputs(&self) -> Vec<InputPinId> {
        match self.typ {
            NodeType::Add(AddData { input1, input2, .. }) => vec![input1, input2],
            NodeType::Multiply(MultData { input1, input2, .. }) => vec![input1, input2],
            NodeType::Sine(SineData { input, .. }) => vec![input],
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

// Helper to get predecessor values for a specific node's inputs
// Returns values in the order of the node's input pins
fn get_input_values(graph: &Graph, node: &Node) -> Vec<f32> {
    let inputs = node.get_inputs();
    inputs
        .iter()
        .map(|&input_pin| {
            // Find the link that ends at this input pin
            graph
                .links
                .iter()
                .find(|link| link.end == input_pin)
                .and_then(|link| {
                    // Find the node that has the output for this link
                    graph
                        .nodes
                        .iter()
                        .find(|n| n.has_output(link.start))
                        .map(|n| n.value)
                })
                .unwrap_or(0.0)
        })
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

    // Now that predecessors are updated, get input values in correct order
    let input_values = get_input_values(graph, &node_clone);

    // Get mutable access to the current node *after* processing predecessors
    if let Some(curr_node) = graph.nodes.get_mut(node_idx) {
        match curr_node.typ {
            NodeType::Add(_) => {
                curr_node.value = input_values.iter().sum();
            }
            NodeType::Multiply(_) => {
                curr_node.value = input_values.iter().product();
            }
            NodeType::Sine(_) => {
                // Sine usually takes one input
                curr_node.value = input_values
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
                // Value is set via UI, just mark as updated
                // (value was already set when the node was created or via slider)
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
    input1: InputPinId,
    input2: InputPinId,
    output: OutputPinId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MultData {
    input1: InputPinId,
    input2: InputPinId,
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

impl<'a> State<'a> {
    pub fn new(context: &'a Context) -> Self {
        let editor_context = context.create_editor(&EditorContextDesc::default());
        let mut id_gen = editor_context.new_identifier_generator();
        let nodes = Graph::new(&mut id_gen);

        Self {
            id_gen,
            editor_context,
            graph: nodes,
            positions_initialized: false,
        }
    }
}

/// https://github.com/Nelarius/imnodes/blob/master/example/color_node_editor.cpp
pub fn show<'a>(ui: &imgui::Ui, state: &mut State<'a>) {
    ui.text("press \"A\" or right click to add a Node");

    // Update graph values before getting colors
    update_output_node(&mut state.graph, 0); // Assuming node 0 is always the output node

    let State {
        editor_context,
        graph,
        id_gen,
        positions_initialized,
        ..
    } = state;

    // Ensure context is current for set_position
    let _ = editor_context.set_as_current_editor();

    // Keep output node locked at right side
    let width = ui.window_content_region_max()[0] - ui.window_content_region_min()[0];
    let _ = graph.nodes[0]
        .id
        .set_position(0.9 * width, 300.0, imnodes::CoordinateSystem::ScreenSpace)
        .set_draggable(false);

    // Determine background color
    let bg_color = if let NodeType::Output(OutData {
        red, green, blue, ..
    }) = &graph.nodes[0].typ
    {
        [*red, *green, *blue]
    } else {
        unreachable!()
    };

    // Draw a colored rectangle as background to show the computed color
    let draw_list = ui.get_window_draw_list();
    let pos = ui.cursor_screen_pos();
    let size = ui.content_region_avail();
    draw_list
        .add_rect(
            pos,
            [pos[0] + size[0], pos[1] + size[1]],
            [bg_color[0], bg_color[1], bg_color[2], 1.0],
        )
        .filled(true)
        .build();

    let outer_scope = create_the_editor(
        ui,
        editor_context,
        graph,
        id_gen,
        bg_color,
        positions_initialized,
    );

    // user interaction handling
    if let Some(link) = outer_scope.links_created() {
        graph.links.push(Link {
            id: id_gen.next_link(),
            start: link.start_pin,
            end: link.end_pin,
        });
        // Trigger graph update after link creation
        update_output_node(graph, 0);
    }

    if let Some(link) = outer_scope.get_destroyed_link() {
        graph.links.retain(|e| e.id != link);
        // Trigger graph update after link destruction
        update_output_node(graph, 0);
    }

    // Handle deletion
    if ui.is_key_released(imgui::Key::Delete) || ui.is_key_released(imgui::Key::Backspace) {
        let selected_nodes = outer_scope.selected_nodes();
        let selected_links = outer_scope.selected_links();

        if !selected_nodes.is_empty() || !selected_links.is_empty() {
            let mut nodes_to_delete = std::collections::HashSet::new();
            for node_id in selected_nodes {
                nodes_to_delete.insert(node_id);
            }

            let mut links_to_delete = std::collections::HashSet::new();
            for link_id in selected_links {
                links_to_delete.insert(link_id);
            }

            // Find links connected to deleted nodes
            for node in &graph.nodes {
                if nodes_to_delete.contains(&node.id) {
                    for link in &graph.links {
                        if node.has_output(link.start) || node.owns_input(link.end) {
                            links_to_delete.insert(link.id);
                        }
                    }
                }
            }

            graph.nodes.retain(|n| !nodes_to_delete.contains(&n.id));
            graph.links.retain(|l| !links_to_delete.contains(&l.id));

            // Trigger graph update
            update_output_node(graph, 0);
        }
    }
}

/// main node ui
fn create_the_editor<'a>(
    ui: &imgui::Ui,
    editor_context: &mut EditorContext<'a>,
    graph: &mut Graph,
    id_gen: &mut IdentifierGenerator,
    bg_color: [f32; 3],
    positions_initialized: &mut bool,
) -> imnodes::OuterScope {
    editor(editor_context, |mut editor| {
        // Apply styles that were previously applied via builder
        editor.push_style(
            &StyleDesc {
                colors: &[
                    (imnodes::ColorStyle::GridBackground, bg_color.into()),
                    (
                        imnodes::ColorStyle::TitleBar,
                        [11.0 / 255.0, 109.0 / 255.0, 191.0 / 255.0].into(),
                    ),
                    (
                        imnodes::ColorStyle::TitleBarHovered,
                        [45.0 / 255.0, 126.0 / 255.0, 194.0 / 255.0].into(),
                    ),
                    (
                        imnodes::ColorStyle::TitleBarSelected,
                        [81.0 / 255.0, 148.0 / 255.0, 204.0 / 255.0].into(),
                    ),
                    (imnodes::ColorStyle::Link, [0.8, 0.5, 0.1].into()),
                ],
                flags: &[
                    AttributeFlags::EnableLinkCreationOnSnap,
                    AttributeFlags::EnableLinkDetachWithDragClick,
                ],
                ..Default::default()
            },
            |editor| {
                // Set initial positions for demo nodes (only once)
                if !*positions_initialized && graph.nodes.len() >= 6 {
                    let _ = graph.nodes[0].id.set_position(
                        760.0,
                        150.0,
                        imnodes::CoordinateSystem::EditorSpace,
                    );
                    let _ = graph.nodes[1].id.set_position(
                        50.0,
                        150.0,
                        imnodes::CoordinateSystem::EditorSpace,
                    );
                    let _ = graph.nodes[2].id.set_position(
                        220.0,
                        150.0,
                        imnodes::CoordinateSystem::EditorSpace,
                    );
                    let _ = graph.nodes[3].id.set_position(
                        400.0,
                        150.0,
                        imnodes::CoordinateSystem::EditorSpace,
                    );
                    let _ = graph.nodes[4].id.set_position(
                        220.0,
                        280.0,
                        imnodes::CoordinateSystem::EditorSpace,
                    );
                    let _ = graph.nodes[5].id.set_position(
                        580.0,
                        150.0,
                        imnodes::CoordinateSystem::EditorSpace,
                    );
                    *positions_initialized = true;
                }

                editor.add_mini_map(&MiniMapDesc {
                    size_fraction: 0.2,
                    location: MiniMapLocation::BottomLeft,
                });

                let popup_id = ui.push_id("popup_add_node");

                // Context menu logic
                if editor.is_hovered()
                    && (ui.is_mouse_clicked(imgui::MouseButton::Right)
                        || ui.is_key_released(imgui::Key::A))
                {
                    ui.open_popup("popup_add_node");
                }

                // Using standard popup instead of modal for better UX
                if let Some(_popup) = ui.begin_popup("popup_add_node") {
                    let click_pos = ui.io().mouse_pos;

                    // Helper closure to generate node ID and set position
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

                    if ui.menu_item("Add") {
                        let node_id = gen_node_base(id_gen);
                        let input_pin1 = id_gen.next_input_pin();
                        let input_pin2 = id_gen.next_input_pin();
                        let output_pin = id_gen.next_output_pin();
                        graph.nodes.push(Node {
                            id: node_id,
                            value: 0.0,
                            typ: NodeType::Add(AddData {
                                input1: input_pin1,
                                input2: input_pin2,
                                output: output_pin,
                            }),
                            updated: false,
                        });
                    }
                    if ui.menu_item("Multiply") {
                        let node_id = gen_node_base(id_gen);
                        let input_pin1 = id_gen.next_input_pin();
                        let input_pin2 = id_gen.next_input_pin();
                        let output_pin = id_gen.next_output_pin();
                        graph.nodes.push(Node {
                            id: node_id,
                            value: 0.0,
                            typ: NodeType::Multiply(MultData {
                                input1: input_pin1,
                                input2: input_pin2,
                                output: output_pin,
                            }),
                            updated: false,
                        });
                    }
                    if ui.menu_item("Sine") {
                        graph.nodes.push(Node {
                            id: gen_node_base(id_gen),
                            value: 0.0,
                            typ: NodeType::Sine(SineData {
                                input: id_gen.next_input_pin(),
                                output: id_gen.next_output_pin(),
                            }),
                            updated: false,
                        });
                    }
                    if ui.menu_item("Time") {
                        graph.nodes.push(Node {
                            id: gen_node_base(id_gen),
                            value: 0.0,
                            typ: NodeType::Time(TimeData {
                                output: id_gen.next_output_pin(),
                            }),
                            updated: false,
                        });
                    }
                    if ui.menu_item("Constant") {
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
                    }
                }
                popup_id.pop();

                for i in 0..graph.nodes.len() {
                    let node_id = graph.nodes[i].id;
                    let node_value = graph.nodes[i].value;
                    // Clone node type to get IDs, but we'll read fresh data for Output node
                    let node_type = graph.nodes[i].typ.clone();

                    editor.add_node(node_id, |mut node_scope| {
                        match &node_type {
                            NodeType::Add(AddData {
                                input1,
                                input2,
                                output,
                                ..
                            }) => {
                                node_scope.add_titlebar(|| ui.text("Add"));
                                node_scope.add_input(
                                    &InputAttributeDesc {
                                        id: *input1,
                                        shape: PinShape::QuadFilled,
                                    },
                                    || ui.text("a"),
                                );
                                node_scope.add_input(
                                    &InputAttributeDesc {
                                        id: *input2,
                                        shape: PinShape::QuadFilled,
                                    },
                                    || ui.text("b"),
                                );
                                ui.text(format!("Value: {:.2}", node_value));
                                node_scope.add_output(
                                    &OutputAttributeDesc {
                                        id: *output,
                                        shape: PinShape::CircleFilled,
                                    },
                                    || ui.text("output"),
                                );
                            }
                            NodeType::Multiply(MultData {
                                input1,
                                input2,
                                output,
                                ..
                            }) => {
                                node_scope.add_titlebar(|| ui.text("Multiply"));
                                node_scope.add_input(
                                    &InputAttributeDesc {
                                        id: *input1,
                                        shape: PinShape::QuadFilled,
                                    },
                                    || ui.text("a"),
                                );
                                node_scope.add_input(
                                    &InputAttributeDesc {
                                        id: *input2,
                                        shape: PinShape::QuadFilled,
                                    },
                                    || ui.text("b"),
                                );
                                ui.text(format!("Value: {:.2}", node_value));
                                node_scope.add_output(
                                    &OutputAttributeDesc {
                                        id: *output,
                                        shape: PinShape::CircleFilled,
                                    },
                                    || ui.text("output"),
                                );
                            }
                            NodeType::Output(OutData {
                                input_red,
                                input_green,
                                input_blue,
                                ..
                            }) => {
                                node_scope.add_titlebar(|| ui.text("Output"));
                                node_scope.add_input(
                                    &InputAttributeDesc {
                                        id: *input_red,
                                        shape: PinShape::QuadFilled,
                                    },
                                    || ui.text("red"),
                                );
                                node_scope.add_input(
                                    &InputAttributeDesc {
                                        id: *input_green,
                                        shape: PinShape::QuadFilled,
                                    },
                                    || ui.text("green"),
                                );
                                node_scope.add_input(
                                    &InputAttributeDesc {
                                        id: *input_blue,
                                        shape: PinShape::QuadFilled,
                                    },
                                    || ui.text("blue"),
                                );
                                // Read FRESH values from graph (not the cloned data)
                                if let NodeType::Output(OutData {
                                    red, green, blue, ..
                                }) = &graph.nodes[i].typ
                                {
                                    ui.text(format!("red: {:.2}", red));
                                    ui.text(format!("green: {:.2}", green));
                                    ui.text(format!("blue: {:.2}", blue));
                                }
                            }
                            NodeType::Sine(SineData { input, output, .. }) => {
                                node_scope.add_titlebar(|| ui.text("Sine"));
                                node_scope.add_input(
                                    &InputAttributeDesc {
                                        id: *input,
                                        shape: PinShape::QuadFilled,
                                    },
                                    || ui.text("input"),
                                );
                                ui.text(format!("Value: {:.2}", node_value));
                                node_scope.add_output(
                                    &OutputAttributeDesc {
                                        id: *output,
                                        shape: PinShape::CircleFilled,
                                    },
                                    || ui.text("output"),
                                );
                            }
                            NodeType::Time(TimeData { output, .. }) => {
                                node_scope.add_titlebar(|| ui.text("Time"));
                                ui.text(format!("Value: {:.2}", node_value));
                                node_scope.add_output(
                                    &OutputAttributeDesc {
                                        id: *output,
                                        shape: PinShape::CircleFilled,
                                    },
                                    || ui.text("output"),
                                );
                            }
                            NodeType::Constant(ConstData {
                                attribute, output, ..
                            }) => {
                                node_scope.add_titlebar(|| ui.text("Constant"));
                                // Need mutable access *only* for the slider
                                let is_active = node_scope.add_static_attribute(
                                    &StaticAttributeDesc { id: *attribute },
                                    || {
                                        ui.set_next_item_width(130.0);
                                        // Get mutable access just for the slider build call
                                        if let Some(node_mut) = graph.nodes.get_mut(i)
                                            && ui
                                                .slider_config("value", 0.0, 1.0)
                                                .display_format(format!("{:.2}", node_mut.value)) // Display current value
                                                .build(&mut node_mut.value)
                                        // Modify actual value
                                        {
                                            // Trigger graph update if constant value changes
                                            update_output_node(graph, 0);
                                        }
                                    },
                                );

                                // Example usage of the new return value:
                                if is_active {
                                    ui.text("Attribute is active!"); // Display message
                                }

                                node_scope.add_output(
                                    &OutputAttributeDesc {
                                        id: *output,
                                        shape: PinShape::CircleFilled,
                                    },
                                    || ui.text("output"),
                                );
                            }
                        } // end match
                    }); // end add_node
                } // end for

                for Link { id, start, end } in &graph.links {
                    editor.add_link(*id, *end, *start);
                }
            },
        );
    })
}
