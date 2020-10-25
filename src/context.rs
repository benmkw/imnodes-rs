use parking_lot::ReentrantMutex;

use crate::sys;
// use crate::NodeUi;

/// TODO, should this be just implicit?
pub struct Context {
    // raw: *mut sys::EditorContext,
}

lazy_static::lazy_static! {
    static ref CTX_MUTEX: ReentrantMutex<()> = ReentrantMutex::new(());
}

impl Context {
    /// create global context
    pub fn new() -> Self {
        let _guard = CTX_MUTEX.lock();

        // let ctx = unsafe { sys::imnodes_EditorContextCreate() };
        // unsafe { sys::imnodes_EditorContextSet(ctx) };
        unsafe { sys::imnodes_Initialize() }

        // Self { raw: ctx }
        Self {}
    }

    // pub fn get_nodes_ui(&self) -> NodeUi {
    //     NodeUi { context: self }
    // }

    /// GetStyle
    /// TODO see Steyle_destroy, make sure this does not leak
    pub fn get_style(&self) -> &mut sys::Style {
        unsafe { &mut *(sys::igGetStyle() as *mut sys::Style) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let _guard = CTX_MUTEX.lock();

        unsafe { sys::imnodes_Shutdown() }

        // unsafe {
        // sys::imnodes_EditorContextFree(self.raw);
        // }
    }
}
