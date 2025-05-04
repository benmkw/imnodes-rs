/*!
This module contains all the scopes for building the node editor UI.

Scopes ensure that `imnodes` functions are called in the correct order as required
by the underlying immediate-mode C++ library. For example, you can only add attributes
inside a node scope, which itself must be inside an editor scope.

Each function that enters a new scope (like `editor`, `add_node`, `add_input`) takes `&mut self`
on the parent scope's struct to prevent calling methods from the parent scope while inside the nested one.
*/

use crate::{
    AttributeId, EditorContext, Hoverable, InputPinId, Link, LinkId, MiniMapLocation, NodeId,
    OutputPinId, PinId, PinShape, sys,
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

    /// Checks if the user dropped a link originating from the given pin ID without connecting it.
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
        // Safety: C API call. Assumes editor context is valid.
        let num = unsafe { sys::imnodes_NumSelectedNodes() };
        // Number can be 0, assertion removed.
        num.max(0) as u32
    }

    /// Returns the number of currently selected links.
    #[doc(alias = "NumSelectedLinks")]
    #[must_use]
    pub fn num_selected_links(&self) -> u32 {
        // Safety: C API call. Assumes editor context is valid.
        let num = unsafe { sys::imnodes_NumSelectedLinks() };
        // Number can be 0, assertion removed.
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
        // Safety: C API call. Writes up to `nr_nodes` IDs into the vector's buffer.
        // `set_len` is safe because the memory is initialized by the C call.
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
        // Safety: C API call. Writes up to `nr_links` IDs into the vector's buffer.
        // `set_len` is safe because the memory is initialized by the C call.
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
        // Note: The bool pointer is const in the C++ definition, but the C binding expects *mut.
        // We initialize it and assume imnodes won't write through this specific pointer.
        let mut created_from_snap: bool = false;

        // Safety: C API call. All pointers are valid mutable references or a bool pointer.
        let is_created = unsafe {
            sys::imnodes_IsLinkCreated_IntPtr(
                core::ptr::from_mut(&mut start_node_id),
                core::ptr::from_mut(&mut start_pin_id),
                core::ptr::from_mut(&mut end_node_id),
                core::ptr::from_mut(&mut end_pin_id),
                // Casting *mut bool is necessary due to bindgen's C interpretation.
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
                // We assume the start is always an output and the end is always an input.
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
        // Safety: C API call. `id` is potentially written to.
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
        // Safety: C API call. `id` is potentially written to.
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
        // Safety: C API call. `id` is potentially written to.
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
        // Safety: C API call. `id` is potentially written to.
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
        // Safety: C API call. `id` is potentially written to.
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
        // Safety: C API call. `id` is potentially written to.
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
/// Requires the [`EditorContext`] to be set via [`EditorContext::set_as_current_editor`] beforehand.
#[doc(alias = "BeginNodeEditor", alias = "EndNodeEditor")]
pub fn editor<F: FnOnce(EditorScope)>(context: &mut EditorContext, f: F) -> OuterScope {
    // Ensure the context is set (though the user should ideally do this explicitly)
    let _ = context.set_as_current_editor();

    // Safety: Begins the editor scope. Must be paired with EndNodeEditor.
    unsafe { sys::imnodes_BeginNodeEditor() };
    f(EditorScope {});
    // Safety: Ends the editor scope.
    unsafe { sys::imnodes_EndNodeEditor() };
    OuterScope {}
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
    /// * `size_fraction`: The size of the minimap relative to the editor canvas (e.g., 0.2 for 20%).
    /// * `location`: The corner where the minimap should be placed.
    #[doc(alias = "MiniMap")]
    pub fn add_mini_map(&mut self, size_fraction: f32, location: MiniMapLocation) {
        // The C API allows a callback, but wrapping it safely with Rust closures
        // and void pointers is complex. We omit it for now.
        let node_hovering_callback = None;
        let node_hovering_callback_data = core::ptr::null_mut::<core::ffi::c_void>();

        // Safety: C API call within the editor scope.
        unsafe {
            sys::imnodes_MiniMap(
                size_fraction,
                location as i32,
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
        // Safety: Begins a node scope. Must be paired with EndNode.
        unsafe { sys::imnodes_BeginNode(id.into()) }
        f(NodeScope {});
        // Safety: Ends the node scope.
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
        // Safety: C API call within the editor scope. Assumes pin IDs are valid.
        // The C API takes (link_id, start_pin, end_pin). We assume start=output, end=input based on common usage.
        unsafe { sys::imnodes_Link(id.into(), output_pin_id.into(), input_pin_id.into()) }
    }

    /// Checks if any attribute's UI is currently active (being interacted with).
    ///
    /// Returns the ID of the active attribute if one exists, otherwise `None`.
    /// This is useful for detecting changes in attribute UIs like sliders or drag floats.
    #[doc(alias = "IsAnyAttributeActive")]
    #[must_use]
    pub fn get_active_attribute(&self) -> Option<AttributeId> {
        let mut id: i32 = -1;
        // Safety: C API call within the editor scope. `id` is potentially written to.
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
        // Safety: C API call within the editor scope.
        unsafe { sys::imnodes_IsEditorHovered() }
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
        // Safety: Begins the title bar scope within a node. Must be paired with EndNodeTitleBar.
        unsafe { sys::imnodes_BeginNodeTitleBar() }
        f();
        // Safety: Ends the title bar scope.
        unsafe { sys::imnodes_EndNodeTitleBar() }
    }

    /// Adds an input pin (rendered on the left side) and its associated attribute UI to the node.
    ///
    /// Place ImGui UI elements for the attribute within the closure `f`.
    ///
    /// # Arguments
    /// * `id`: A unique identifier for this input pin.
    /// * `shape`: The visual shape of the pin.
    /// * `f`: A closure that defines the UI content associated with this pin.
    #[doc(alias = "BeginInputAttribute", alias = "EndInputAttribute")]
    pub fn add_input<F: FnOnce()>(&mut self, id: InputPinId, shape: PinShape, f: F) {
        // Safety: Begins an input attribute scope. Must be paired with EndInputAttribute.
        unsafe { sys::imnodes_BeginInputAttribute(id.into(), shape as i32) };
        f();
        // Safety: Ends the input attribute scope.
        unsafe { sys::imnodes_EndInputAttribute() };
    }

    /// Adds an output pin (rendered on the right side) and its associated attribute UI to the node.
    ///
    /// Place ImGui UI elements for the attribute within the closure `f`.
    ///
    /// # Arguments
    /// * `id`: A unique identifier for this output pin.
    /// * `shape`: The visual shape of the pin.
    /// * `f`: A closure that defines the UI content associated with this pin.
    #[doc(alias = "BeginOutputAttribute", alias = "EndOutputAttribute")]
    pub fn add_output<F: FnOnce()>(&mut self, id: OutputPinId, shape: PinShape, f: F) {
        // Safety: Begins an output attribute scope. Must be paired with EndOutputAttribute.
        unsafe { sys::imnodes_BeginOutputAttribute(id.into(), shape as i32) };
        f();
        // Safety: Ends the output attribute scope.
        unsafe { sys::imnodes_EndOutputAttribute() };
    }

    /// Adds a static attribute (UI element without a pin) to the node.
    ///
    /// Static attributes cannot be linked. Place ImGui UI elements for the attribute within the closure `f`.
    /// Use [`crate::helpers::is_last_attribute_active()`] or [`EditorScope::get_active_attribute()`]
    /// to check for interaction with the UI defined in `f`.
    ///
    /// # Arguments
    /// * `id`: A unique identifier for this static attribute.
    /// * `f`: A closure that defines the UI content of the attribute.
    #[doc(alias = "BeginStaticAttribute", alias = "EndStaticAttribute")]
    pub fn add_static_attribute<F: FnOnce()>(&mut self, id: AttributeId, f: F) {
        // Safety: Begins a static attribute scope. Must be paired with EndStaticAttribute.
        unsafe { sys::imnodes_BeginStaticAttribute(id.into()) };
        f();
        // Safety: Ends the static attribute scope.
        unsafe { sys::imnodes_EndStaticAttribute() };
    }
}
