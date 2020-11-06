use std::collections::HashMap;

#[test]
fn exc_1() {
    struct Stats {
        mean: f64,
        median: Option<f64>,
        mode: Option<i32>,
    }

    fn calculate_stats(mut values: Vec<i32>) -> Stats {
        let mut mean = 0.0;
        let mut map = HashMap::new();
        values.sort_unstable();
        for (i, x) in values.iter().enumerate() {
            let mut entry = map.entry(x).or_insert(0);
            *entry += 1;

            mean += ((*x as f64 - mean) / (i + 1) as f64);
        }

        let median = {
            match values.len() {
                0 => None,
                n if (n % 2 == 0) => {
                    let i = (n / 2);
                    Some((values[(i - 1)] + values[i]) as f64 / 2.0)
                }
                n => {
                    let i = (((n - 1) as f64) / 2.0).ceil() as usize;
                    Some(values[i] as f64)
                }
            }
        };

        let mode = map
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _)| *k);

        Stats { mean, median, mode }
    }

    assert_eq!(calculate_stats(vec![1, 2, 3, 4, 5]).mean, 3.0);
    assert_eq!(calculate_stats(vec![1, 2, 2, 3, 4, 5]).mode, Some(2));
    assert_eq!(calculate_stats(vec![1, 2, 3, 4, 5]).median, Some(3.0));
    assert_eq!(calculate_stats(vec![1, 2, 3, 4, 4, 5]).median, Some(3.5));
}

fn exc_2() {
    fn pig_latin(str: &str) -> Option<String> {
        let lowercase = str.to_lowercase();
        let mut chars = lowercase.chars();
        match chars.next()? {
            'a' | 'e' | 'i' | 'o' | 'u' => Some(format!("{}-hay", lowercase)),
            n => {
                let remaining: String = chars.collect();
                Some(format!("{}-{}ay", remaining, n))
            }
        }
    }
    assert_eq!(pig_latin("first"), Some("irst-fay".to_string()));
    assert_eq!(pig_latin("apple"), Some("apple-hay".to_string()));
}
