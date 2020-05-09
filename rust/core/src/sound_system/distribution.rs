use super::Distribution;
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

pub fn frequency(letters: &[(&'_ str, f64)]) -> Vec<(String, f64)> {
    let is_equal = letters
        .first()
        .map(|(_, first)| {
            letters
                .iter()
                .fold((true, first), |(is_same, previous), (_, weight)| {
                    (
                        is_same && (*weight - *previous).abs() < std::f64::EPSILON,
                        weight,
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
            .map(|(i, (letter, _))| {
                let frequency = gusein_zade(i, size);
                ((*letter).to_string(), frequency)
            })
            .collect()
    } else {
        letters
            .iter()
            .map(|(letter, weight)| ((*letter).to_string(), *weight))
            .collect()
    }
}

pub fn select(frequency: &'_ [Distribution]) -> &'_ str {
    let sum = frequency.iter().map(|t| t.1).sum::<f64>();
    let mut tmp = 0.0;
    let picked = thread_rng().gen_range(0.0, sum);
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
