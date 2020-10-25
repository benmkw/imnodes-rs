// TOOO look at https://github.com/4bb4/implot-rs/blob/master/src/context.rs and see if this approach makes
// sense here too

use crate::sys;

/// TODO, should this be just implicit?
pub struct Context {
    // raw: *mut sys::EditorContext,
}

impl Context {
    /// create global context
    pub fn new() -> Self {
        // let ctx = unsafe { sys::imnodes_EditorContextCreate() };
        // unsafe { sys::imnodes_EditorContextSet(ctx) };
        unsafe { sys::imnodes_Initialize() }

        Self {}
    }

    // pub fn get_nodes_ui(&self) -> NodeUi {
    //     NodeUi { context: self }
    // }

    /// GetStyle
    /// TODO see Steyle_destroy, make sure this does not leak
    pub fn get_style(&self) -> &mut sys::Style {
        unsafe { &mut *(sys::imnodes_GetStyle() as *mut sys::Style) }
    }

    /// generate Singleton IdentifierGenerator
    pub fn new_identifier_generator(&self) -> crate::IdentifierGenerator {
        crate::IdentifierGenerator::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::imnodes_Shutdown() }

        // unsafe {
        // sys::imnodes_EditorContextFree(self.raw);
        // }
    }
}
