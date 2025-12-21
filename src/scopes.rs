/*!
This module contains all the scopes for building the node editor UI.

Scopes ensure that `imnodes` functions are called in the correct order as required
by the underlying immediate-mode C++ library. For example, you can only add attributes
inside a node scope, which itself must be inside an editor scope.

Each function that enters a new scope (like `editor`, `add_node`, `add_input`) takes `&mut self`
on the parent scope's struct to prevent calling methods from the parent scope while inside the nested one.
*/

use crate::{
    AttributeFlags, AttributeId, ColorStyle, EditorContext, Hoverable, InputPinId, Link, LinkId,
    MiniMapLocation, NodeId, OutputPinId, PinId, PinShape, StyleVar, sys,
};

/// Represents the scope outside the main node editor block.
/// Use methods on this struct *after* [`editor()`] has returned to query UI events.
#[derive(Debug)]
pub struct OuterScope {}
impl OuterScope {
    /// Checks if a specific hoverable UI element (node, pin, or link) is currently hovered by the mouse.
    #[doc(
        alias = "IsPinHovered",
        alias = "IsNodeHovered",
        alias = "IsLinkHovered"
    )]
    #[must_use]
    pub fn is_hovered(&self, id: impl Hoverable) -> bool {
        id.is_hovered(self)
    }

    /// Checks if the user started dragging a new link from the given pin ID in this frame.
    #[doc(alias = "IsLinkStarted")]
    #[must_use]
    pub fn link_started_at<T: Into<PinId>>(&self, id: T) -> bool {
        let id: PinId = id.into();
        id.is_start_of_link(self)
    }

    /// Checks if the user dropped a dragged link originating from this pin without connecting it.
    ///
    /// # Arguments
    /// * `id`: The pin ID from which the link might have been dropped.
    /// * `including_detached_links`: If `true`, also returns `true` if an existing link was detached from this pin and then dropped. If `false`, only triggers for newly created links that are dropped.
    #[doc(alias = "IsLinkDropped")]
    #[must_use]
    pub fn link_dropped_from<T: Into<PinId>>(&self, id: T, including_detached_links: bool) -> bool {
        let id: PinId = id.into();
        id.dropped_link(including_detached_links, self)
    }

    /// Returns the number of currently selected nodes.
    #[doc(alias = "NumSelectedNodes")]
    #[must_use]
    pub fn num_selected_nodes(&self) -> u32 {
        let num = unsafe { sys::imnodes_NumSelectedNodes() };
        num.max(0) as u32
    }

    /// Returns the number of currently selected links.
    #[doc(alias = "NumSelectedLinks")]
    #[must_use]
    pub fn num_selected_links(&self) -> u32 {
        let num = unsafe { sys::imnodes_NumSelectedLinks() };
        num.max(0) as u32
    }

    /// Returns a vector containing the IDs of all currently selected nodes.
    #[doc(alias = "GetSelectedNodes")]
    #[must_use]
    pub fn selected_nodes(&self) -> Vec<NodeId> {
        let nr_nodes = self.num_selected_nodes();
        if nr_nodes == 0 {
            return Vec::new();
        }
        let mut nodes: Vec<NodeId> = Vec::with_capacity(nr_nodes as usize);
        unsafe {
            sys::imnodes_GetSelectedNodes(nodes.as_mut_ptr().cast::<i32>());
            nodes.set_len(nr_nodes as usize);
        }
        nodes
    }

    /// Returns a vector containing the IDs of all currently selected links.
    #[doc(alias = "GetSelectedLinks")]
    #[must_use]
    pub fn selected_links(&self) -> Vec<LinkId> {
        let nr_links = self.num_selected_links();
        if nr_links == 0 {
            return Vec::new();
        }
        let mut links: Vec<LinkId> = Vec::with_capacity(nr_links as usize);
        unsafe {
            sys::imnodes_GetSelectedLinks(links.as_mut_ptr().cast::<i32>());
            links.set_len(nr_links as usize);
        }
        links
    }

    /// Checks if the user finished creating a new link in this frame.
    ///
    /// Returns `Some(Link)` containing details about the new link if one was created,
    /// otherwise `None`.
    #[doc(alias = "IsLinkCreated", alias = "IsLinkCreated_IntPtr")]
    #[must_use]
    pub fn links_created(&self) -> Option<Link> {
        let mut start_node_id: i32 = -1;
        let mut start_pin_id: i32 = -1;
        let mut end_node_id: i32 = -1;
        let mut end_pin_id: i32 = -1;
        let mut created_from_snap: bool = false;

        let is_created = unsafe {
            sys::imnodes_IsLinkCreated_IntPtr(
                core::ptr::from_mut(&mut start_node_id),
                core::ptr::from_mut(&mut start_pin_id),
                core::ptr::from_mut(&mut end_node_id),
                core::ptr::from_mut(&mut end_pin_id),
                core::ptr::from_mut::<bool>(&mut created_from_snap).cast(),
            )
        };

        if is_created
            && start_pin_id >= 0
            && end_pin_id >= 0
            && start_node_id >= 0
            && end_node_id >= 0
        {
            Some(Link {
                start_node: NodeId { id: start_node_id },
                end_node: NodeId { id: end_node_id },
                start_pin: OutputPinId { id: start_pin_id },
                end_pin: InputPinId { id: end_pin_id },
                created_from_snap,
            })
        } else {
            None
        }
    }

    /// Checks if an existing link was detached (destroyed) by the user in this frame.
    ///
    /// Returns `Some(LinkId)` of the destroyed link if one was destroyed, otherwise `None`.
    #[doc(alias = "IsLinkDestroyed")]
    #[must_use]
    pub fn get_destroyed_link(&self) -> Option<LinkId> {
        let mut id: i32 = -1;
        if unsafe { sys::imnodes_IsLinkDestroyed(core::ptr::from_mut(&mut id)) } && id >= 0 {
            Some(LinkId { id })
        } else {
            None
        }
    }

    /// Gets the ID of the pin currently being hovered over, if any.
    #[doc(alias = "IsPinHovered")]
    #[must_use]
    pub fn get_hovered_pin(&self) -> Option<PinId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsPinHovered(core::ptr::from_mut(&mut id)) };
        if ok && id >= 0 {
            Some(PinId { id })
        } else {
            None
        }
    }

    /// Gets the ID of the link currently being hovered over, if any.
    #[doc(alias = "IsLinkHovered")]
    #[must_use]
    pub fn get_hovered_link(&self) -> Option<LinkId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsLinkHovered(core::ptr::from_mut(&mut id)) };
        if ok && id >= 0 {
            Some(LinkId { id })
        } else {
            None
        }
    }

    /// Gets the ID of the attribute whose UI is currently active (being interacted with), if any.
    #[doc(alias = "IsAnyAttributeActive")]
    #[must_use]
    pub fn get_active_attribute(&self) -> Option<AttributeId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsAnyAttributeActive(core::ptr::from_mut(&mut id)) };
        if ok && id >= 0 {
            Some(AttributeId { id })
        } else {
            None
        }
    }

    /// Gets the ID of the pin from which a new link drag was started in this frame, if any.
    #[doc(alias = "IsLinkStarted")]
    #[must_use]
    pub fn from_where_link_started(&self) -> Option<PinId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsLinkStarted(core::ptr::from_mut(&mut id)) };
        if ok && id >= 0 {
            Some(PinId { id })
        } else {
            None
        }
    }

    /// Gets the ID of the pin from which a link drag was dropped in this frame, if any.
    ///
    /// # Arguments
    /// * `including_detached_links`: If `true`, also returns the pin ID if an existing link was detached and dropped.
    #[doc(alias = "IsLinkDropped")]
    #[must_use]
    pub fn from_where_link_dropped(&self, including_detached_links: bool) -> Option<PinId> {
        let mut id: i32 = -1;
        let ok = unsafe {
            sys::imnodes_IsLinkDropped(core::ptr::from_mut(&mut id), including_detached_links)
        };
        if ok && id >= 0 {
            Some(PinId { id })
        } else {
            None
        }
    }
}

/// Begins the node editor UI definition.
///
/// Call methods on the provided [`EditorScope`] to add nodes and links.
/// The function returns an [`OuterScope`] which can be used *after* this function returns
/// to query events like link creation or destruction.
///
/// This function automatically sets the provided context as the current editor context
/// before beginning the editor UI.
///
/// ```no_run
/// # use imnodes::{EditorContext, editor};
/// # let mut context: EditorContext = unimplemented!();
/// // No need to call set_as_current_editor() - editor() handles it!
/// let outer = editor(&mut context, |mut scope| {
///     // add nodes, links, etc.
/// });
/// ```
#[doc(alias = "BeginNodeEditor", alias = "EndNodeEditor")]
pub fn editor<F: FnOnce(EditorScope)>(context: &EditorContext<'_>, f: F) -> OuterScope {
    // Set the global current context via FFI using the provided `EditorContext`.
    // We inline the unsafe FFI call here using the crate-visible `raw` pointer.
    // Public APIs that mutate the context still require `&mut EditorContext`.
    unsafe { sys::imnodes_EditorContextSet(context.raw) };
    unsafe { sys::imnodes_BeginNodeEditor() };
    f(EditorScope {});
    unsafe { sys::imnodes_EndNodeEditor() };
    OuterScope {}
}

/// Descriptor for configuring the minimap.
#[derive(Debug, Clone)]
pub struct MiniMapDesc {
    /// The size of the minimap relative to the editor canvas (e.g., 0.2 for 20%).
    pub size_fraction: f32,
    /// The corner where the minimap should be placed.
    pub location: MiniMapLocation,
}

impl Default for MiniMapDesc {
    fn default() -> Self {
        Self {
            size_fraction: 0.2,
            location: MiniMapLocation::TopLeft,
        }
    }
}

/// Descriptor for pushing multiple style modifications.
#[derive(Debug, Clone, Default)]
pub struct StyleDesc<'a> {
    /// Style variables to push.
    pub vars: &'a [StyleVar],
    /// Colors to push.
    pub colors: &'a [(ColorStyle, imgui::ImColor32)],
    /// Attribute flags to push.
    pub flags: &'a [AttributeFlags],
}

/// Represents the scope within the main node editor block (`imnodes::editor`).
/// Use methods on this struct to add nodes, links, and the minimap.
#[derive(Debug)]
pub struct EditorScope {}
impl EditorScope {
    /// Adds an interactive minimap overlay to the editor canvas.
    ///
    /// Must be called just before the end of the [`editor`] closure.
    ///
    /// # Arguments
    /// * `desc`: A [`MiniMapDesc`] struct specifying the minimap's configuration.
    #[doc(alias = "MiniMap")]
    pub fn add_mini_map(&mut self, desc: &MiniMapDesc) {
        // The C API allows a callback, but wrapping it safely with Rust closures
        // and void pointers is complex. We omit it for now.
        let node_hovering_callback = None;
        let node_hovering_callback_data = core::ptr::null_mut::<core::ffi::c_void>();

        unsafe {
            sys::imnodes_MiniMap(
                desc.size_fraction,
                desc.location as i32,
                node_hovering_callback,
                node_hovering_callback_data,
            );
        }
    }

    /// Adds a node to the editor.
    ///
    /// Call methods on the provided [`NodeScope`] within the closure `f` to define the node's content
    /// (title bar, attributes, pins).
    ///
    /// # Arguments
    /// * `id`: A unique identifier for this node.
    /// * `f`: A closure that defines the content of the node.
    #[doc(alias = "BeginNode", alias = "EndNode")]
    pub fn add_node<F: FnOnce(NodeScope)>(&mut self, id: NodeId, f: F) {
        unsafe { sys::imnodes_BeginNode(id.id) }
        f(NodeScope {});
        unsafe {
            sys::imnodes_EndNode();
        }
    }

    /// Renders a link between an output pin and an input pin.
    ///
    /// The pin IDs must match the IDs used when creating the pins with [`NodeScope::add_input`]
    /// and [`NodeScope::add_output`].
    ///
    /// # Arguments
    /// * `id`: A unique identifier for this link.
    /// * `input_pin_id`: The ID of the input pin (usually on the destination node).
    /// * `output_pin_id`: The ID of the output pin (usually on the source node).
    #[doc(alias = "Link")]
    pub fn add_link(&mut self, id: LinkId, input_pin_id: InputPinId, output_pin_id: OutputPinId) {
        unsafe { sys::imnodes_Link(id.id, output_pin_id.id, input_pin_id.id) }
    }

    /// Checks if any attribute's UI is currently active (being interacted with).
    ///
    /// Returns the ID of the active attribute if one exists, otherwise `None`.
    /// This is useful for detecting changes in attribute UIs like sliders or drag floats.
    #[doc(alias = "IsAnyAttributeActive")]
    #[must_use]
    pub fn get_active_attribute(&self) -> Option<AttributeId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsAnyAttributeActive(core::ptr::from_mut(&mut id)) };
        if ok && id >= 0 {
            Some(AttributeId { id })
        } else {
            None
        }
    }

    /// Checks if the node editor canvas itself is being hovered over by the mouse.
    /// Returns `false` if the mouse is over a node, link, pin, or another ImGui window.
    #[doc(alias = "IsEditorHovered")]
    #[must_use]
    pub fn is_hovered(&self) -> bool {
        unsafe { sys::imnodes_IsEditorHovered() }
    }

    /// Pushes multiple style modifications onto the stack for the duration of the closure.
    ///
    /// # Arguments
    /// * `desc`: A [`StyleDesc`] struct describing the styles to push.
    /// * `f`: The closure to execute with the applied styles.
    pub fn push_style<F: FnOnce(&mut Self)>(&mut self, desc: &StyleDesc<'_>, f: F) {
        unsafe {
            for flag in desc.flags {
                sys::imnodes_PushAttributeFlag(*flag as i32);
            }
            for var in desc.vars {
                match var {
                    StyleVar::GridSpacing(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_GridSpacing as i32,
                        *v,
                    ),
                    StyleVar::NodeCornerRounding(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_NodeCornerRounding as i32,
                        *v,
                    ),
                    StyleVar::NodePadding(v) => sys::imnodes_PushStyleVar_Vec2(
                        sys::ImNodesStyleVar__ImNodesStyleVar_NodePadding as i32,
                        sys::ImVec2_c { x: v[0], y: v[1] },
                    ),
                    StyleVar::NodeBorderThickness(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_NodeBorderThickness as i32,
                        *v,
                    ),
                    StyleVar::LinkThickness(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_LinkThickness as i32,
                        *v,
                    ),
                    StyleVar::LinkLineSegmentsPerLength(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_LinkLineSegmentsPerLength as i32,
                        *v,
                    ),
                    StyleVar::LinkHoverDistance(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_LinkHoverDistance as i32,
                        *v,
                    ),
                    StyleVar::PinCircleRadius(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinCircleRadius as i32,
                        *v,
                    ),
                    StyleVar::PinQuadSideLength(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinQuadSideLength as i32,
                        *v,
                    ),
                    StyleVar::PinTriangleSideLength(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinTriangleSideLength as i32,
                        *v,
                    ),
                    StyleVar::PinLineThickness(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinLineThickness as i32,
                        *v,
                    ),
                    StyleVar::PinHoverRadius(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinHoverRadius as i32,
                        *v,
                    ),
                    StyleVar::PinOffset(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinOffset as i32,
                        *v,
                    ),
                    StyleVar::MiniMapPadding(v) => sys::imnodes_PushStyleVar_Vec2(
                        sys::ImNodesStyleVar__ImNodesStyleVar_MiniMapPadding as i32,
                        sys::ImVec2_c { x: v[0], y: v[1] },
                    ),
                    StyleVar::MiniMapOffset(v) => sys::imnodes_PushStyleVar_Vec2(
                        sys::ImNodesStyleVar__ImNodesStyleVar_MiniMapOffset as i32,
                        sys::ImVec2_c { x: v[0], y: v[1] },
                    ),
                }
            }
            for (style, color) in desc.colors {
                let c: imgui::ImColor32 = *color;
                sys::imnodes_PushColorStyle(*style as i32, c.into());
            }
        }

        f(self);

        unsafe {
            for _ in 0..desc.colors.len() {
                sys::imnodes_PopColorStyle();
            }
            if !desc.vars.is_empty() {
                sys::imnodes_PopStyleVar(desc.vars.len() as i32);
            }
            for _ in 0..desc.flags.len() {
                sys::imnodes_PopAttributeFlag();
            }
        }
    }
}

/// Descriptor for adding an input attribute.
#[derive(Debug, Clone)]
pub struct InputAttributeDesc {
    /// Unique identifier for the input pin.
    pub id: InputPinId,
    /// Visual shape of the pin.
    pub shape: PinShape,
}

impl InputAttributeDesc {
    /// Creates a new descriptor.
    pub fn new(id: InputPinId, shape: PinShape) -> Self {
        Self { id, shape }
    }
}

/// Descriptor for adding an output attribute.
#[derive(Debug, Clone)]
pub struct OutputAttributeDesc {
    /// Unique identifier for the output pin.
    pub id: OutputPinId,
    /// Visual shape of the pin.
    pub shape: PinShape,
}

impl OutputAttributeDesc {
    /// Creates a new descriptor.
    pub fn new(id: OutputPinId, shape: PinShape) -> Self {
        Self { id, shape }
    }
}

/// Descriptor for adding a static attribute.
#[derive(Debug, Clone)]
pub struct StaticAttributeDesc {
    /// Unique identifier for the attribute.
    pub id: AttributeId,
}

impl StaticAttributeDesc {
    /// Creates a new descriptor.
    pub fn new(id: AttributeId) -> Self {
        Self { id }
    }
}

/// Represents the scope within a node definition block (`add_node`).
/// Use methods on this struct to add title bars, input/output pins, and static attributes.
#[derive(Debug)]
pub struct NodeScope {}
impl NodeScope {
    /// Adds a title bar to the node.
    ///
    /// Place ImGui UI elements for the title bar (e.g., `ui.text("Title")`) within the closure `f`.
    /// This *must* be called before adding any attributes or pins using other `add_*` methods
    /// on the `NodeScope`.
    #[doc(alias = "BeginNodeTitleBar", alias = "EndNodeTitleBar")]
    pub fn add_titlebar<F: FnOnce()>(&mut self, f: F) {
        unsafe { sys::imnodes_BeginNodeTitleBar() }
        f();
        unsafe { sys::imnodes_EndNodeTitleBar() }
    }

    /// Adds an input pin (rendered on the left side) and its associated attribute UI to the node.
    ///
    /// Place ImGui UI elements for the attribute within the closure `f`.
    /// Returns `true` if the attribute's UI is currently active (e.g. being clicked/dragged).
    ///
    /// # Arguments
    /// * `desc`: Descriptor containing the ID and shape of the input pin.
    /// * `f`: A closure that defines the UI content associated with this pin.
    #[doc(alias = "BeginInputAttribute", alias = "EndInputAttribute")]
    pub fn add_input<F: FnOnce()>(&mut self, desc: &InputAttributeDesc, f: F) -> bool {
        unsafe { sys::imnodes_BeginInputAttribute(desc.id.id, desc.shape as i32) };
        f();
        unsafe { sys::imnodes_EndInputAttribute() };
        unsafe { sys::imnodes_IsAttributeActive() }
    }

    /// Adds an output pin (rendered on the right side) and its associated attribute UI to the node.
    ///
    /// Place ImGui UI elements for the attribute within the closure `f`.
    /// Returns `true` if the attribute's UI is currently active (e.g. being clicked/dragged).
    ///
    /// # Arguments
    /// * `desc`: Descriptor containing the ID and shape of the output pin.
    /// * `f`: A closure that defines the UI content associated with this pin.
    #[doc(alias = "BeginOutputAttribute", alias = "EndOutputAttribute")]
    pub fn add_output<F: FnOnce()>(&mut self, desc: &OutputAttributeDesc, f: F) -> bool {
        unsafe { sys::imnodes_BeginOutputAttribute(desc.id.id, desc.shape as i32) };
        f();
        unsafe { sys::imnodes_EndOutputAttribute() };
        unsafe { sys::imnodes_IsAttributeActive() }
    }

    /// Adds a static attribute (UI element without a pin) to the node.
    ///
    /// Static attributes cannot be linked. Place ImGui UI elements for the attribute within the closure `f`.
    /// Returns `true` if the attribute's UI is currently active (e.g. being clicked/dragged).
    ///
    /// # Arguments
    /// * `desc`: Descriptor containing the ID of the attribute.
    /// * `f`: A closure that defines the UI content of the attribute.
    #[doc(alias = "BeginStaticAttribute", alias = "EndStaticAttribute")]
    pub fn add_static_attribute<F: FnOnce()>(&mut self, desc: &StaticAttributeDesc, f: F) -> bool {
        unsafe { sys::imnodes_BeginStaticAttribute(desc.id.id) };
        f();
        unsafe { sys::imnodes_EndStaticAttribute() };
        unsafe { sys::imnodes_IsAttributeActive() }
    }

    /// Pushes multiple style modifications onto the stack for the duration of the closure.
    ///
    /// # Arguments
    /// * `desc`: A [`StyleDesc`] struct describing the styles to push.
    /// * `f`: The closure to execute with the applied styles.
    pub fn push_style<F: FnOnce(&mut Self)>(&mut self, desc: &StyleDesc<'_>, f: F) {
        unsafe {
            for flag in desc.flags {
                sys::imnodes_PushAttributeFlag(*flag as i32);
            }
            for var in desc.vars {
                match var {
                    StyleVar::GridSpacing(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_GridSpacing as i32,
                        *v,
                    ),
                    StyleVar::NodeCornerRounding(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_NodeCornerRounding as i32,
                        *v,
                    ),
                    StyleVar::NodePadding(v) => sys::imnodes_PushStyleVar_Vec2(
                        sys::ImNodesStyleVar__ImNodesStyleVar_NodePadding as i32,
                        sys::ImVec2_c { x: v[0], y: v[1] },
                    ),
                    StyleVar::NodeBorderThickness(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_NodeBorderThickness as i32,
                        *v,
                    ),
                    StyleVar::LinkThickness(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_LinkThickness as i32,
                        *v,
                    ),
                    StyleVar::LinkLineSegmentsPerLength(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_LinkLineSegmentsPerLength as i32,
                        *v,
                    ),
                    StyleVar::LinkHoverDistance(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_LinkHoverDistance as i32,
                        *v,
                    ),
                    StyleVar::PinCircleRadius(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinCircleRadius as i32,
                        *v,
                    ),
                    StyleVar::PinQuadSideLength(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinQuadSideLength as i32,
                        *v,
                    ),
                    StyleVar::PinTriangleSideLength(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinTriangleSideLength as i32,
                        *v,
                    ),
                    StyleVar::PinLineThickness(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinLineThickness as i32,
                        *v,
                    ),
                    StyleVar::PinHoverRadius(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinHoverRadius as i32,
                        *v,
                    ),
                    StyleVar::PinOffset(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinOffset as i32,
                        *v,
                    ),
                    StyleVar::MiniMapPadding(v) => sys::imnodes_PushStyleVar_Vec2(
                        sys::ImNodesStyleVar__ImNodesStyleVar_MiniMapPadding as i32,
                        sys::ImVec2_c { x: v[0], y: v[1] },
                    ),
                    StyleVar::MiniMapOffset(v) => sys::imnodes_PushStyleVar_Vec2(
                        sys::ImNodesStyleVar__ImNodesStyleVar_MiniMapOffset as i32,
                        sys::ImVec2_c { x: v[0], y: v[1] },
                    ),
                }
            }
            for (style, color) in desc.colors {
                let c: imgui::ImColor32 = *color;
                sys::imnodes_PushColorStyle(*style as i32, c.into());
            }
        }

        f(self);

        unsafe {
            for _ in 0..desc.colors.len() {
                sys::imnodes_PopColorStyle();
            }
            if !desc.vars.is_empty() {
                sys::imnodes_PopStyleVar(desc.vars.len() as i32);
            }
            for _ in 0..desc.flags.len() {
                sys::imnodes_PopAttributeFlag();
            }
        }
    }
}
