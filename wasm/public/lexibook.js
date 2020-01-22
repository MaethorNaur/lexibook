import * as wasm from "lexibook-wasm";
export const parse = async input => wasm.SoundSystem.parse(input);

export const generate_words = async (
  sound_system,
  number_of_words,
  repartition
) =>
  new Promise((resolve, reject) => {
    try {
      resolve(sound_system.generate_words(number_of_words, repartition));
    } catch (e) {
      reject(e);
    }
  });

export const sound_trasformation = async (sound_system, words) =>
  new Promise((resolve, reject) => {
    try {
      resolve(sound_system.sound_trasformation(words));
    } catch (e) {
      reject(e);
    }
  });
