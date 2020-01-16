#![allow(non_snake_case)]
#[cfg(feature = "console_error_panic_hook")]
extern crate console_error_panic_hook;

extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate wee_alloc;

use wasm_bindgen::prelude::*;
mod utils;
use lexibook::sound_system;
use lexibook::sound_system::rules;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();

    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
}

#[wasm_bindgen(js_name = SoundSystem)]
pub struct JSSoundSystem {
    sound_system: sound_system::SoundSystem,
}

#[wasm_bindgen(js_name = Transformation)]
pub struct JSTransformation {
    result: rules::Transformation,
}

#[wasm_bindgen(js_class = Transformation)]
impl JSTransformation {
    pub fn output(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.result.output).unwrap()
    }
    pub fn history(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.result.history).unwrap()
    }
}

#[wasm_bindgen(js_class = SoundSystem)]
impl JSSoundSystem {
    pub async fn parse(js_input: String) -> Result<JSSoundSystem, JsValue> {
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

    pub fn sound_trasformation(&mut self, js_words: Vec<JsValue>) -> JSTransformation {
        let words = js_words.into_iter().filter_map(|v| v.as_string()).collect();
        let result = self.sound_system.sound_trasformation(words);
        JSTransformation { result }
    }
    pub fn get_ipa(&self, word: String) -> String {
        self.sound_system.ipa_representation(&word)
    }
}
