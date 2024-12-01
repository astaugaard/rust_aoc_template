use std::fs;

use once_cell::sync::Lazy;

use crate::day::Day;

pub fn golden<'a, Input>(
    file: &'a str,
    parent: &'a Lazy<Day<Input>>,
    expected_a: Option<&'a str>,
    expected_b: Option<&'a str>,
) -> Box<dyn Fn(bool) -> Option<String> + 'a + Send + Sync> {
    Box::new(move |verbose| {
        let input = match fs::read_to_string(format!("goldens/{}", file)) {
            Ok(a) => a,
            Err(_) => return Some(format!("golden {} failed: could not open file", file)),
        };

        let parent = Lazy::force(parent);

        let input = match (*parent.parser)(input, verbose) {
            Ok(a) => a,
            Err(err) => return Some(format!("golden {} failed to parse: {}", file, err)),
        };

        let part_a = (*parent.part_a)(&input);
        let part_b = (*parent.part_b)(&input);

        if let Some(_a) = expected_a {
            if expected_a != part_a.as_deref() {
                return Some(format!(
                    "golden {} expected {:?} got {:?}",
                    file, expected_a, part_a
                ));
            }
        }

        if let Some(_a) = expected_b {
            if expected_b != part_b.as_deref() {
                return Some(format!(
                    "golden {} expected {:?} got {:?}",
                    file, expected_b, part_b
                ));
            }
        }

        None
    })
}
