#[macro_use]
extern crate criterion;

use lexibook::sound_system::MonoSyllableRepartition;

use criterion::*;

const INPUT: &'static str = r"import:  std-assimilations ./src/test
letters: l, h, t, a, b, d, e, o, y, w, ä, ë, ö, m, n, p, t, c, g, x

phonemes:
  th /θ/ 
  ä /ja/ 
  ë /je/
  ö /jo/
  c /k/
  h /h/ at the beginning of word 

syllables: CV V RV

rules:
_C: w ->
C_#: w ->
Vn ~> Ṽ
aa ~> aː
ee ~> eː
V_V: S -> Z
";
fn run_without_rule(numbers: usize) -> Vec<String> {
    lexibook::sound_system::from_string(INPUT)
        .map(|sound_system| {
            sound_system.generate_words(numbers, MonoSyllableRepartition::LessFrequent)
        })
        .unwrap()
}

fn run_with_rule(numbers: usize) -> Vec<String> {
    lexibook::sound_system::from_string(INPUT)
        .map(|mut sound_system| {
            let words = sound_system.generate_words(numbers, MonoSyllableRepartition::LessFrequent);
            let result = sound_system.sound_trasformation(words);
            result.output
        })
        .unwrap()
}

fn group<F>(c: &mut Criterion, name: &str, f: F)
where
    F: Fn(usize) -> Vec<String>,
{
    let mut group = c.benchmark_group(name);
    for numbers in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*numbers as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(numbers),
            numbers,
            |b, &numbers| b.iter(|| f(numbers)),
        );
    }
    group.finish();
}
fn bench(c: &mut Criterion) {
    group(c, "without_rule", run_without_rule);
    group(c, "with_rule", run_with_rule);
}
criterion_group!(benches, bench);
criterion_main!(benches);
