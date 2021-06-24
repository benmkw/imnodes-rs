/*!
This module contains all the scopes.
The cpp code requires that certain methods may only be called in certain scopes.

As soon as you enter a nested scope you are not allowed to call methods from the other scope inside the nested one.
This is why every method which takes a closure and calls it with a new scope takes `&mut self`.
*/

use crate::{
    sys, AttributeId, EditorContext, Hoverable, InputPinId, Link, LinkId, NodeId, OutputPinId,
    PinId, PinShape,
};

/// entry point
#[doc(alias = "BeginNodeEditor", alias = "EndNodeEditor")]
pub fn editor<F: FnOnce(EditorScope)>(context: &mut EditorContext, f: F) -> OuterScope {
    context.set_as_current_editor();

    unsafe { sys::imnodes_BeginNodeEditor() };
    f(EditorScope {});
    unsafe { sys::imnodes_EndNodeEditor() };
    OuterScope {}
}

/// Original Scopes turned into compile time checks:
/// Scope_None = 1,
#[derive(Debug)]
pub struct OuterScope {}
impl OuterScope {
    /// check the position of the mosue
    #[doc(
        alias = "IsPinHovered",
        alias = "IsNodeHovered",
        alias = "IsLinkHovered"
    )]
    pub fn is_hovered(&self, id: impl Hoverable) -> bool {
        id.is_hovered(self)
    }

    /// Did the user start dragging a new link from a pin?
    #[doc(alias = "IsLinkStarted")]
    pub fn link_started_at<T: Into<PinId>>(&self, id: T) -> bool {
        let id: PinId = id.into();
        id.is_start_of_link(self)
    }

    /// Did the user drop the dragged link before attaching it to a pin?
    /// There are two different kinds of situations to consider when handling this event:
    /// 1) a link which is created at a pin and then dropped
    /// 2) an existing link which is detached from a pin and then dropped
    ///
    /// Use the including_detached_links flag to control whether this function triggers when the user
    /// detaches a link and drops it.
    #[doc(alias = "IsLinkDropped")]
    pub fn link_dropped_from<T: Into<PinId>>(&self, id: T, including_detached_links: bool) -> bool {
        let id: PinId = id.into();
        id.dropped_link(including_detached_links, self)
    }

    /// Query the number of selected nodes in the current editor.
    ///
    /// selected_nodes builds on top of this
    #[doc(alias = "NumSelectedNodes")]
    pub fn num_selected_nodes(&self) -> u32 {
        let num = unsafe { sys::imnodes_NumSelectedNodes() };
        assert!(num > 0);
        num as u32
    }

    /// Query the number of selected links in the current editor.
    ///
    /// selected_links builds on top of this
    #[doc(alias = "NumSelectedLinks")]
    pub fn num_selected_links(&self) -> u32 {
        let num = unsafe { sys::imnodes_NumSelectedLinks() };
        assert!(num > 0);
        num as u32
    }

    /// Get the selected node ids.
    #[doc(alias = "GetSelectedNodes")]
    pub fn selected_nodes(&self) -> Vec<NodeId> {
        let nr_nodes = self.num_selected_nodes() as usize;
        let mut nodes = vec![NodeId { id: 0 }; nr_nodes];
        unsafe { sys::imnodes_GetSelectedNodes(nodes.as_mut_ptr() as _) };
        nodes
    }

    /// Get the selected link ids.
    #[doc(alias = "GetSelectedLinks")]
    pub fn selected_links(&self) -> Vec<LinkId> {
        let nr_links = self.num_selected_links() as usize;
        let mut links = vec![LinkId { id: 0 }; nr_links];
        unsafe { sys::imnodes_GetSelectedLinks(links.as_mut_ptr() as _) };
        links
    }

    /// Did the user finish creating a new link?
    #[doc(alias = "IsLinkCreated")]
    pub fn links_created(&self) -> Option<Link> {
        let mut started_at_node_id: i32 = -1;
        let mut started_at_attribute_id: i32 = -1;
        let mut ended_at_node_id: i32 = -1;
        let mut ended_at_attribute_id: i32 = -1;
        let mut created_from_snap: bool = true;

        let is_created = unsafe {
            sys::imnodes_IsLinkCreated_IntPtr(
                &mut started_at_node_id as _,
                &mut started_at_attribute_id as _,
                &mut ended_at_node_id as _,
                &mut ended_at_attribute_id as _,
                &mut created_from_snap as *mut bool,
            )
        };

        // let is_created = unsafe {
        //     sys::imnodes_IsLinkCreatedBoolPtr(
        //         &mut started_at_attribute_id as _,
        //         &mut ended_at_attribute_id as _,
        //         &mut created_from_snap as *mut bool,
        //     )
        // };

        if is_created {
            Some(Link {
                start_node: NodeId {
                    id: started_at_node_id,
                },
                end_node: NodeId {
                    id: ended_at_node_id,
                },
                start_pin: OutputPinId {
                    id: started_at_attribute_id,
                },
                end_pin: InputPinId {
                    id: ended_at_attribute_id,
                },
                craeated_from_snap: created_from_snap,
            })
        } else {
            None
        }
    }

    /// Was an existing link detached from a pin by the user? The detached link's id is assigned to the output argument link_id.
    #[doc(alias = "IsLinkDestroyed")]
    pub fn get_dropped_link(&self) -> Option<LinkId> {
        let mut id: i32 = -1;
        if unsafe { sys::imnodes_IsLinkDestroyed(&mut id as _) } {
            Some(LinkId { id })
        } else {
            None
        }
    }

    /// IsPinHovered
    #[doc(alias = "IsPinHovered")]
    pub fn get_hovered_pin(&self) -> Option<PinId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsPinHovered(&mut id as _) };
        if ok {
            Some(PinId { id })
        } else {
            None
        }
    }

    /// IsLinkHovered
    #[doc(alias = "IsLinkHovered")]
    pub fn get_hovered_link(&self) -> Option<LinkId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsLinkHovered(&mut id as _) };
        if ok {
            Some(LinkId { id })
        } else {
            None
        }
    }

    /// IsAnyAttributeActive
    #[doc(alias = "IsAnyAttributeActive")]
    pub fn get_active_attribute(&self) -> Option<AttributeId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsAnyAttributeActive(&mut id as _) };
        if ok {
            Some(AttributeId { id })
        } else {
            None
        }
    }

    /// IsLinkStarted
    #[doc(alias = "IsLinkStarted")]
    pub fn from_where_link_started(&self) -> Option<PinId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsLinkStarted(&mut id as _) };
        if ok {
            Some(PinId { id })
        } else {
            None
        }
    }

    /// IsLinkDropped
    #[doc(alias = "IsLinkDropped")]
    pub fn from_where_link_dropped(&self, including_detached_links: bool) -> Option<PinId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsLinkDropped(&mut id as _, including_detached_links) };
        if ok {
            Some(PinId { id })
        } else {
            None
        }
    }
}

/// Scope_Editor = 1 << 1,
#[derive(Debug)]
pub struct EditorScope {}
impl EditorScope {
    /// BeginNode
    /// ...
    /// EndNode
    #[doc(alias = "BeginNode", alias = "EndNode")]
    pub fn add_node<F: FnOnce(NodeScope)>(&mut self, id: NodeId, f: F) {
        unsafe { sys::imnodes_BeginNode(id.into()) }

        f(NodeScope {});
        unsafe { sys::imnodes_EndNode() };
    }

    /// Render a link between attributes.
    ///
    /// The attributes ids used here must match the ids used in Begin(Input|Output)Attribute function
    /// calls. The order of start_attr and end_attr doesn't make a difference for rendering the link.
    #[doc(alias = "Link")]
    pub fn add_link(&self, id: LinkId, input: InputPinId, output: OutputPinId) {
        unsafe { sys::imnodes_Link(id.into(), input.into(), output.into()) }
    }

    /// IsAnyAttributeActive
    #[doc(alias = "IsAnyAttributeActive")]
    pub fn get_active_attribute(&self) -> Option<AttributeId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsAnyAttributeActive(&mut id as _) };
        if ok {
            Some(AttributeId { id })
        } else {
            None
        }
    }

    /// Returns true if the current node editor canvas is being hovered over by the mouse, and is not blocked by any other windows.
    #[doc(alias = "IsEditorHovered")]
    pub fn is_hovered(&self) -> bool {
        unsafe { sys::imnodes_IsEditorHovered() }
    }
}

/// Scope_Node = 1 << 2,
#[derive(Debug)]
pub struct NodeScope {}
impl NodeScope {
    /// Place your node title bar content (such as the node title, using [imgui::Ui::text]) between the
    /// following function calls. These functions have to be called before adding any attributes, or the
    /// layout of the node will be incorrect.
    ///
    /// TODO enforce that titlebar is created before attributes are added? Add one more state maybe?
    #[doc(alias = "BeginNodeTitleBar", alias = "EndNodeTitleBar")]
    pub fn add_titlebar<F: FnOnce()>(&mut self, f: F) {
        unsafe { sys::imnodes_BeginNodeTitleBar() }
        f();
        unsafe { sys::imnodes_EndNodeTitleBar() }
    }

    /// Attributes are ImGui UI elements embedded within the node. Attributes can have pin shapes
    /// rendered next to them. Links are created between pins.
    ///
    /// The activity status of an attribute can be checked via the [crate::helpers::is_last_attribute_active()] and
    /// [OuterScope::get_active_attribute()] function calls. This is one easy way of checking for any changes made to
    /// an attribute's drag float UI, for instance.
    ///
    // Each attribute id must be unique.
    ///
    /// Create an input attribute block. The pin is rendered on left side.
    #[doc(alias = "BeginInputAttribute", alias = "EndInputAttribute")]
    pub fn add_input<F: FnOnce()>(&mut self, id: InputPinId, shape: PinShape, f: F) {
        unsafe { sys::imnodes_BeginInputAttribute(id.into(), shape as u32) };
        f();
        unsafe { sys::imnodes_EndInputAttribute() };
    }

    /// Attributes are ImGui UI elements embedded within the node. Attributes can have pin shapes
    /// rendered next to them. Links are created between pins.
    ///
    /// The activity status of an attribute can be checked via the [crate::helpers::is_last_attribute_active()] and
    /// [OuterScope::get_active_attribute()] function calls. This is one easy way of checking for any changes made to
    /// an attribute's drag float UI, for instance.
    ///
    // Each attribute id must be unique.
    ///
    /// Create an output attribute block. The pin is rendered on the right side.
    #[doc(alias = "BeginOutputAttribute", alias = "EndOutputAttribute")]
    pub fn add_output<F: FnOnce()>(&mut self, id: OutputPinId, shape: PinShape, f: F) {
        unsafe { sys::imnodes_BeginOutputAttribute(id.into(), shape as u32) };
        f();
        unsafe { sys::imnodes_EndOutputAttribute() };
    }

    /// Create a static attribute block. A static attribute has no pin, and therefore can't be linked to anything.
    /// However, you can still use [crate::helpers::is_last_attribute_active()] and [OuterScope::get_active_attribute()] to check for attribute activity.
    #[doc(alias = "BeginStaticAttribute", alias = "EndStaticAttribute")]
    pub fn attribute<F: FnOnce()>(&mut self, id: AttributeId, f: F) {
        unsafe { sys::imnodes_BeginStaticAttribute(id.into()) };
        f();
        unsafe { sys::imnodes_EndStaticAttribute() };
    }
}
