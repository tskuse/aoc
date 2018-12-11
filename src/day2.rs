use std::collections::BTreeMap;

pub fn checksum<I>(values: I) -> i32
where
    I: Iterator<Item = String>,
{
    let res = values.fold((0, 0), |mut acc, x| {
        if has_char_freq(&x, 2) {
            acc.0 += 1;
        }
        if has_char_freq(&x, 3) {
            acc.1 += 1;
        }
        acc
    });
    res.0 * res.1
}

fn has_one_char_diff(a: &String, b: &String) -> bool {
    let chars_diff_count = a.chars().zip(b.chars()).fold(0, |acc, (c1, c2)| {
        if c1 != c2 {
            return acc + 1;
        }
        acc
    });
    chars_diff_count == 1
}

pub fn strings_with_one_char_diff(values: &[String]) -> Vec<String> {
    // O(n^2) time
    let mut stack: Vec<String> = Vec::new();
    for c1 in values {
        for c2 in values {
            if has_one_char_diff(&c1, &c2) {
                stack.push(c1.to_string());
            }
        }
    }
    stack
}

pub fn has_char_freq(input: &String, freq: u32) -> bool {
    let mut chars = BTreeMap::new();
    for c in input.chars() {
        if let Some(freq) = chars.get_mut(&c) {
            *freq += 1;
        } else {
            chars.insert(c, 1);
        }
    }
    for &value in chars.values() {
        if value == freq {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_two_letters() {
        assert_eq!(true, has_char_freq(&"hah".to_string(), 2));
    }

    #[test]
    fn it_does_not_have_two_letters() {
        assert_eq!(false, has_char_freq(&"abbbcdefg".to_string(), 2));
    }

    #[test]
    fn it_has_three_letters() {
        assert_eq!(true, has_char_freq(&"blahbsdb".to_string(), 3));
    }

    #[test]
    fn it_does_not_have_three_letters() {
        assert_eq!(false, has_char_freq(&"abbbab".to_string(), 3));
    }

    #[test]
    fn it_has_one_char_diff() {
        assert_eq!(
            true,
            has_one_char_diff(
                &"qzsayvpigkzmrdawijuefotxbh".to_string(),
                &"qzsayvpigkzmrdawijuefotxbz".to_string()
            )
        )
    }

    #[test]
    fn it_does_not_have_one_char_diff() {
        assert_eq!(
            false,
            has_one_char_diff(
                &"qcsnyvpigkzmrdapljuyfotxih".to_string(),
                &"qcsnhlpigkzmrtawljuefotxbh".to_string()
            )
        )
    }

    #[test]
    fn it_returns_strings_with_one_char_diff() {
        let input: Vec<String> = vec![
            "abcdefg".to_string(),
            "fdamlfm".to_string(),
            "dkeocml".to_string(),
            "abcxefg".to_string(),
        ];
        let expected = vec!["abcdefg".to_string(), "abcxefg".to_string()];
        let output: Vec<String> = strings_with_one_char_diff(&input);
        assert_eq!(output.eq(&expected), true);
    }
}
