use lexibook::sound_system::SoundSystem;
use std::os::raw::c_void;

/// Create a new souns system
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn lexibook_sound_system_new() -> *mut c_void {
    let sound_system = SoundSystem::with_default();
    Box::into_raw(Box::new(sound_system)) as *mut c_void
}
