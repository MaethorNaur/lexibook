use rand::prelude::*;
pub fn power_law(max: usize, percentage: f32) -> usize {
    let mut result = 0;
    while random::<f32>() >= percentage {
        result = (result + 1) % max;
    }
    result
}

fn gusein_zade(position: usize, size: usize) -> f64 {
    (((size + 1) as f64).log10() - ((position + 1) as f64).log10()) / (size as f64) * 100.0
}

pub fn frequency(letters: Vec<&'_ str>) -> Vec<(String, f64)> {
    let size = letters.len();
    letters
        .iter()
        .enumerate()
        .map(|(i, letter)| {
            let frequency = gusein_zade(i, size);
            ((*letter).to_string(), frequency)
        })
        .collect()
}

pub fn select(frequency: Vec<&(String, f64)>) -> &'_ str {
    let sum = frequency.iter().map(|t| t.1).sum::<f64>();
    let mut tmp = 0.0;
    let picked = thread_rng().gen_range(0.0, sum);
    for tuple in &frequency {
        let (letter, weight) = tuple;
        tmp += weight;
        if picked < tmp {
            return letter;
        }
    }
    &frequency[0].0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_frequency() {
        let letters = vec!["e", "a", "c"];
        let result = frequency(letters);
        println!("{:#?}", result);
    }
}
