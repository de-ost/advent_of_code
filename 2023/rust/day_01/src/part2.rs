pub fn solve_part_2(input: &str) -> u32 {
    input
        .split("\n")
        .fold(0, |acc: u32, line| acc + parse_first_last(line))
}

fn parse_first_last(txt: &str) -> u32 {
    let mut v = (0, 0);
    let mut first_found = false;

    for i in 0..txt.len() {
        let s = &txt[i..];
        if let Some(n) = get_number(s) {
            if !first_found {
                v.0 = n;
                v.1 = n;
                first_found = true;
            } else {
                v.1 = n;
            }
        }
    }

    v.0 * 10 + v.1
}

fn get_number(txt: &str) -> Option<u32> {
    match txt.chars().next() {
        // Normal number
        Some('1'..='9') => txt.chars().next().and_then(|c| c.to_digit(10)),
        // Number written out
        _ => match txt {
            s if s.starts_with("one") => Some(1),
            s if s.starts_with("two") => Some(2),
            s if s.starts_with("three") => Some(3),
            s if s.starts_with("four") => Some(4),
            s if s.starts_with("five") => Some(5),
            s if s.starts_with("six") => Some(6),
            s if s.starts_with("seven") => Some(7),
            s if s.starts_with("eight") => Some(8),
            s if s.starts_with("nine") => Some(9),
            _ => None,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_number_1() {
        assert_eq!(get_number("1asd"), Some(1));
    }

    #[test]
    fn test_get_number_2() {
        assert_eq!(get_number("x"), None);
    }

    #[test]
    fn test_get_number_3() {
        assert_eq!(get_number("one"), Some(1));
    }

    #[test]
    fn test_get_number_4() {
        assert_eq!(get_number("oneag"), Some(1));
    }

    #[test]
    fn test_1() {
        assert_eq!(parse_first_last("two1nine"), 29);
    }

    #[test]
    fn test_2() {
        assert_eq!(parse_first_last("eightwothree"), 83);
    }

    #[test]
    fn test_3() {
        assert_eq!(parse_first_last("abcone2threexyz"), 13);
    }

    #[test]
    fn test_4() {
        assert_eq!(parse_first_last("xtwone3four"), 24);
    }

    #[test]
    fn test_5() {
        assert_eq!(parse_first_last("4nineeightseven2"), 42);
    }

    #[test]
    fn test_6() {
        assert_eq!(parse_first_last("zoneight234"), 14);
    }

    #[test]
    fn test_7() {
        assert_eq!(parse_first_last("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_sum_of_all() {
        let txt = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let sum = txt
            .split("\n")
            .fold(0, |acc: u32, line| acc + parse_first_last(line));
        assert_eq!(sum, 281);
    }
}
