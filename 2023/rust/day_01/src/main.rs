// https://adventofcode.com/2023/day/1

fn main() {
    let input = include_str!("../resources/input.txt");
    let sum = input
        .split("\n")
        .fold(0, |acc: u32, line| acc + parse_first_last(line));

    println!("Sum: {}", sum);
}

fn parse_first_last(txt: &str) -> u32 {
    let mut v = (0, 0);
    let mut first_found = false;

    for c in txt.chars() {
        if let Ok(num) = c.to_string().parse() {
            if !first_found {
                v = (num, num);
                first_found = true;
            } else {
                v.1 = num;
            }
        }
    }

    return 10 * v.0 + v.1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(parse_first_last("1abc2"), 12)
    }

    #[test]
    fn test_2() {
        assert_eq!(parse_first_last("pqr3stu8vwx"), 38)
    }

    #[test]
    fn test_3() {
        assert_eq!(parse_first_last("a1b2c3d4e5f"), 15)
    }

    #[test]
    fn test_4() {
        assert_eq!(parse_first_last("treb7uchet"), 77)
    }

    #[test]
    fn test_sum_of_all() {
        let txt = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        let sum = txt
            .split("\n")
            .fold(0, |acc: u32, line| acc + parse_first_last(line));
        assert_eq!(sum, 142);
    }
}
