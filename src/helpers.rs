use crate::sys;
use crate::sys::*;
use crate::Context;

/// EditorContextGetPanning
pub fn get_panning(_: &Context) -> ImVec2 {
    let mut position = ImVec2 { x: 0.0, y: 0.0 };
    unsafe { sys::imnodes_EditorContextGetPanning(&mut position as _) };
    position
}

/// EditorContextResetPanning
pub fn reset_panning(pos: ImVec2, _: &Context) {
    unsafe { sys::imnodes_EditorContextResetPanning(pos) };
}

/// dark color theme
pub fn set_style_colors_dark(_: &Context) {
    unsafe { sys::imnodes_StyleColorsDark() };
}

/// classic color theme
pub fn set_style_colors_classic(_: &Context) {
    unsafe { sys::imnodes_StyleColorsClassic() };
}

/// light color theme
pub fn set_style_colors_light(_: &Context) {
    unsafe { sys::imnodes_StyleColorsLight() };
}

/// IsEditorHovered
/// no context as param because this is crate internal and mainly for ScopeEditor
pub(crate) fn is_editor_hovered() -> bool {
    unsafe { sys::imnodes_IsEditorHovered() }
}
