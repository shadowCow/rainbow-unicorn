use std::cmp;

pub fn oscillating_proportion(elapsed_time_millis: f64, time_period_millis: f64) -> f64 {
    let proportion = repeating_proportion(elapsed_time_millis, time_period_millis);
    let phase = (elapsed_time_millis / time_period_millis) as u64 % 2;

    if phase == 0 {
        proportion
    } else {
        1.0 - proportion
    }
}

pub fn repeating_proportion(elapsed_time_millis: f64, time_period_millis: f64) -> f64 {
    let remainder = (elapsed_time_millis as u64) % time_period_millis as u64;
    
    remainder as f64 / time_period_millis
}

pub fn proportion(elapsed_time_millis: f64, time_period_millis: f64) -> f64 {
    let quotient = elapsed_time_millis / time_period_millis;
    if quotient > 1.0 {
        1.0
    } else {
        quotient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillating_proportion() {
        let time_period_millis = 1000.0;

        let elapsed_time_expected_proportion_pairs = vec![
            (0.0, 0.0),
            (250.0, 0.25),
            (750.0, 0.75),
            (1000.0, 1.0),
            (1250.0, 0.75),
            (2000.0, 0.0),
            (2500.0, 0.5),
            (3000.0, 1.0)
        ];

        for pair in elapsed_time_expected_proportion_pairs.iter() {
            let actual = oscillating_proportion(pair.0, time_period_millis);
            assert_eq!(pair.1, actual, "elapsed: {}", pair.0);
        }
    }

    #[test]
    fn test_repeating_proportion() {
        let time_period_millis = 500.0;

        let elapsed_time_expected_proportion_pairs = vec![
            (0.0, 0.0),
            (100.0, 0.2),
            (300.0, 0.6),
            (500.0, 0.0),
            (600.0, 0.2),
            (1000.0, 0.0)
        ];

        for pair in elapsed_time_expected_proportion_pairs.iter() {
            let actual = repeating_proportion(pair.0, time_period_millis);
            assert_eq!(pair.1, actual, "elapsed: {}", pair.0);
        }
    }

    #[test]
    fn test_proportion() {
        let time_period_millis = 800.0;

        let elapsed_time_expected_proportion_pairs = vec![
            (0.0, 0.0),
            (100.0, 0.125),
            (700.0, 0.875),
            (800.0, 1.0),
            (1200.0, 1.0),
            (2400.0, 1.0)
        ];

        for pair in elapsed_time_expected_proportion_pairs.iter() {
            let actual = proportion(pair.0, time_period_millis);
            assert_eq!(pair.1, actual, "elapsed: {}", pair.0);
        }
    }
}