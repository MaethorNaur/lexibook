import * as wasm from "lexibook-wasm";
import * as datatable from "vanilla-datatables";
import css from "vanilla-datatables/src/vanilla-dataTables.css";
const input = document.getElementById("input");
const words = document.getElementById("words");
const error = document.getElementById("error");

function toRepartition() {
  const radios = document.getElementsByName("repartition");
  for (var i = 0, length = radios.length; i < length; i++) {
    if (radios[i].checked) {
      switch (radios[i].value) {
        case "Always":
          return wasm.MonoSyllableRepartition.Always;
        case "Mostly":
          return wasm.MonoSyllableRepartition.Mostly;
        case "Frequent":
          return wasm.MonoSyllableRepartition.Frequent;
        case "LessFrequent":
          return wasm.MonoSyllableRepartition.LessFrequent;
        case "Rare":
          return wasm.MonoSyllableRepartition.Rare;
        case "Never":
          return wasm.MonoSyllableRepartition.Never;
      }
    }
  }
}
document.getElementById("gen").addEventListener("click", _ev => {
  let result = document.createElement("table");
  document.querySelectorAll(".dataTable-wrapper").forEach(e => e.remove());
  error.innerHTML = "";
  let number_of_words = parseInt(words.value, 10);
  let repartition = toRepartition();
  try {
    let ss = wasm.SoundSystem.parse(input.value);
    let words = ss.generate_words(number_of_words, repartition);
    let trans = ss.sound_trasformation(words);
    let data = { headings: ["Words"], data: [] };
    trans.history().forEach(t => data.headings.push(t.rule));
    data.headings.push("Final");
    data.headings.push("IPA");

    words.forEach((word, i) => {
      let row = [word];
      trans.history().forEach((rule, j) => {
        let prev = word;
        if (j > 0) {
          prev = trans.history()[j - 1].words[i];
        }
        let current = rule.words[i];
        let w = "--";
        if (prev != current) {
          w = `<b style="color: red">${current}</b>`;
        }
        row.push(w);
      });
      row.push(`<b>${trans.output()[i]}</b>`);
      row.push(`<i>/${ss.get_ipa(trans.output()[i])}/</i>`);
      data.data.push(row);
    });
    document.body.appendChild(result);
    ss.free();
    trans.free();
    new datatable(result, { searchable: false, fixedHeight: true, data });
  } catch (e) {
    console.log(e);
    if (typeof e == "string") {
      error.innerHTML = e.replace("\n", "<br/>");
    } else {
      error.innerHTML = e;
    }
  }
});
