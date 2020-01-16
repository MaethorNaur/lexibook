cfg_if::cfg_if! {
    if #[cfg(feature = "wasm")] {
        mod wasm;
        pub use wasm::*;
    } else {
        mod sys;
        pub use sys::*;
    }
}

mod common;
pub use common::*;
