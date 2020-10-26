use crate::EditorContext;
use imnodes_sys as sys;

/// EditorContextGetPanning
pub fn get_panning(_: &EditorContext) -> sys::ImVec2 {
    let mut position = sys::ImVec2 { x: 0.0, y: 0.0 };
    unsafe { sys::imnodes_EditorContextGetPanning(&mut position as _) };
    position
}

/// EditorContextResetPanning
pub fn reset_panning(pos: sys::ImVec2, _: &EditorContext) {
    unsafe { sys::imnodes_EditorContextResetPanning(pos) };
}

/// IsAttributeActive
pub fn is_last_attribute_active() -> bool {
    unsafe { sys::imnodes_IsAttributeActive() }
}
