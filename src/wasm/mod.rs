#![allow(non_snake_case)]
#[cfg(feature = "console_error_panic_hook")]
extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
mod utils;
use crate::sound_system;
use crate::sound_system::rules;

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    #[cfg(not(debug_assertions))]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Warn));
}

#[wasm_bindgen(js_name = SoundSystem)]
pub struct JSSoundSystem {
    sound_system: sound_system::SoundSystem<'static>,
}

#[wasm_bindgen(js_class = SoundSystem)]
impl JSSoundSystem {
    pub fn parse(js_input: String) -> Result<JSSoundSystem, JsValue> {
        let input = Box::leak(js_input.into_boxed_str());
        sound_system::from_string(input)
            .map(|sound_system| JSSoundSystem { sound_system })
            .map_err(|error| JsValue::from(error.to_string()))
    }
    pub fn generate_words(
        &self,
        number_of_words: u32,
        repartition: sound_system::MonoSyllableRepartition,
    ) -> Vec<JsValue> {
        self.sound_system
            .generate_words(number_of_words as usize, repartition)
            .into_iter()
            .map(JsValue::from)
            .collect()
    }
    pub fn sound_trasformation(&self, js_words: Vec<JsValue>) -> Result<JsValue, JsValue> {
        let words = js_words.into_iter().filter_map(|v| v.as_string()).collect();
        let result = rules::sound_trasformation(&self.sound_system, words);
        JsValue::from_serde(&result).map_err(|error| JsValue::from(error.to_string()))
    }
}
