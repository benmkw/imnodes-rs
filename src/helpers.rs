use crate::EditorContext;
use imnodes_sys as sys;

impl EditorContext {
    /// EditorContextGetPanning
    #[doc(alias = "EditorContextGetPanning")]
    pub fn get_panning(&self) -> sys::ImVec2 {
        let mut position = sys::ImVec2 { x: 0.0, y: 0.0 };
        unsafe { sys::imnodes_EditorContextGetPanning(&mut position as _) };
        position
    }
    /// EditorContextResetPanning
    #[doc(alias = "EditorContextResetPanning")]
    pub fn reset_panning(&self, pos: sys::ImVec2) {
        unsafe { sys::imnodes_EditorContextResetPanning(pos) };
    }

    /// Clears the list of selected nodes/links. Useful if you want to delete a selected node or link.
    #[doc(alias = "ClearNodeSelection")]
    pub fn clear_node_selection(&self) {
        unsafe { sys::imnodes_ClearNodeSelection() };
    }

    /// ClearLinkSelection
    #[doc(alias = "ClearLinkSelection")]
    pub fn clear_link_selection(&self) {
        unsafe { sys::imnodes_ClearLinkSelection() };
    }
}

/// Was the previous attribute active? This will continuously return true while the left mouse button is being pressed over the UI content of the attribute.
#[doc(alias = "IsAttributeActive")]
pub fn is_last_attribute_active() -> bool {
    unsafe { sys::imnodes_IsAttributeActive() }
}
