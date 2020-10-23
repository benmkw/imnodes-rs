extern crate imnodes_sys as sys;

pub use sys::{imgui::Condition, ImVec2, ImVec4, Style};

pub fn create_imnodes_context() {
    unsafe { sys::imnodes_Initialize() }
}

pub fn imnodes_end_context() {
    unsafe { sys::imnodes_Shutdown() }
}
pub fn begin_editor() {
    unsafe { sys::imnodes_BeginNodeEditor() }
}

pub fn end_editor() {
    unsafe { sys::imnodes_EndNodeEditor() }
}

pub fn begin_node(id: i32) {
    unsafe { sys::imnodes_BeginNode(id) }
}

pub fn end_node() {
    unsafe { sys::imnodes_EndNode() }
}

pub fn begin_output(id: i32) {
    // pub const PinShape_PinShape_Circle: PinShape = 0;
    // pub const PinShape_PinShape_CircleFilled: PinShape = 1;
    // pub const PinShape_PinShape_Triangle: PinShape = 2;
    // pub const PinShape_PinShape_TriangleFilled: PinShape = 3;
    // pub const PinShape_PinShape_Quad: PinShape = 4;
    // pub const PinShape_PinShape_QuadFilled: PinShape = 5;
    let shape = 0;
    unsafe { sys::imnodes_BeginOutputAttribute(id, shape) }
}

pub fn end_output() {
    unsafe { sys::imnodes_EndOutputAttribute() }
}

pub fn begin_input(id: i32) {
    // pub const PinShape_PinShape_Circle: PinShape = 0;
    // pub const PinShape_PinShape_CircleFilled: PinShape = 1;
    // pub const PinShape_PinShape_Triangle: PinShape = 2;
    // pub const PinShape_PinShape_TriangleFilled: PinShape = 3;
    // pub const PinShape_PinShape_Quad: PinShape = 4;
    // pub const PinShape_PinShape_QuadFilled: PinShape = 5;
    let shape = 0;
    unsafe { sys::imnodes_BeginInputAttribute(id, shape) }
}

pub fn end_input() {
    unsafe { sys::imnodes_EndInputAttribute() }
}
