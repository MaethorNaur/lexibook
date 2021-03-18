use super::Distribution;
use crate::wgl::Letter;
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

pub fn frequency(letters: &[Letter]) -> Vec<(String, f64)> {
    let is_equal = letters
        .first()
        .map(|first| {
            letters
                .iter()
                .fold((true, first.frequency), |(is_same, previous), current| {
                    (
                        is_same && (current.frequency - previous).abs() < std::f64::EPSILON,
                        current.frequency,
                    )
                })
                .0
        })
        .unwrap_or(true);
    if is_equal {
        let size = letters.len();
        letters
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let frequency = gusein_zade(i, size);
                (c.letter.clone(), frequency)
            })
            .collect()
    } else {
        letters
            .iter()
            .map(|c| (c.letter.clone(), c.frequency))
            .collect()
    }
}

pub fn select(frequency: &'_ [Distribution]) -> &'_ str {
    let sum = frequency.iter().map(|t| t.1).sum::<f64>();
    let mut tmp = 0.0;
    let picked = thread_rng().gen_range(0.0..sum);
    for tuple in frequency {
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
        let letters = vec![("e", 0.0), ("a", 0.0), ("c", 0.0)];
        let result = frequency(&letters);
        assert_eq!(
            result,
            vec![
                ("e".to_string(), 20.068666377598745,),
                ("a".to_string(), 10.034333188799373,),
                ("c".to_string(), 4.164624553609999,),
            ]
        )
    }
}
