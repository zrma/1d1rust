use std::cmp::Ordering;

#[allow(dead_code)]
fn highest_value_palindrome(s: String, n: i32, k: i32) -> String {
    if n == 0 {
        return "-1".to_string();
    }
    if n == 1 {
        if k <= 0 {
            return s;
        }
        return "9".to_string();
    }

    let (b, k, ok) = make_simple_palindrome(s.chars().collect(), n, k);
    if !ok {
        return "-1".to_string();
    }
    make_highest_value_palindrome(s.chars().collect(), b, n, k)
}

fn make_simple_palindrome(mut result: Vec<char>, n: i32, mut k: i32) -> (Vec<char>, i32, bool) {
    let mut i1: usize = 0;
    let mut i2: usize = (n - 1) as usize;
    while i1 < i2 {
        let lhs = result.get(i1).unwrap();
        let rhs = result.get(i2).unwrap();
        match lhs.cmp(rhs) {
            Ordering::Greater => {
                result[i2] = result[i1];
                k -= 1;
                if k < 0 {
                    return (result, -1, false);
                }
            }
            Ordering::Less => {
                result[i1] = result[i2];
                k -= 1;
                if k < 0 {
                    return (result, -1, false);
                }
            }
            _ => {}
        }
        i1 += 1;
        i2 -= 1;
    }
    (result, k, true)
}

fn make_highest_value_palindrome(s: Vec<char>, mut b: Vec<char>, n: i32, mut k: i32) -> String {
    let mut i1: usize = 0;
    let mut i2: usize = (n - 1) as usize;
    while i1 <= i2 {
        if i1 == i2 && k > 0 {
            b[i1] = '9';
        }
        if b[i1] != '9' {
            if (b[i1] == s[i1] && b[i2] == s[i2]) && k >= 2 {
                b[i1] = '9';
                b[i2] = '9';
                k -= 2;
            } else if (b[i1] != s[i1] || b[i2] != s[i2]) && k >= 1 {
                b[i1] = '9';
                b[i2] = '9';
                k -= 1;
            }
        }
        i1 += 1;
        i2 -= 1;
    }
    b.iter().collect()
}

// https://www.hackerrank.com/challenges/richie-rich/problem
#[test]
fn test_highest_value_palindrome() {
    let (extremely_long_given, extremely_long_expected) = read_extremely_long_test_data();

    struct TestData {
        s: String,
        n: i32,
        k: i32,
        expected: String,
    }
    for data in vec![
        TestData {
            s: "3943".to_string(),
            n: 4,
            k: 1,
            expected: "3993".to_string(),
        },
        TestData {
            s: "092282".to_string(),
            n: 6,
            k: 3,
            expected: "992299".to_string(),
        },
        TestData {
            s: "0011".to_string(),
            n: 4,
            k: 1,
            expected: "-1".to_string(),
        },
        TestData {
            s: "1100".to_string(),
            n: 4,
            k: 1,
            expected: "-1".to_string(),
        },
        TestData {
            s: "".to_string(),
            n: 0,
            k: 0,
            expected: "-1".to_string(),
        },
        TestData {
            s: "1".to_string(),
            n: 1,
            k: 0,
            expected: "1".to_string(),
        },
        TestData {
            s: "1".to_string(),
            n: 1,
            k: 1,
            expected: "9".to_string(),
        },
        TestData {
            s: "01".to_string(),
            n: 2,
            k: 1,
            expected: "11".to_string(),
        },
        TestData {
            s: "00".to_string(),
            n: 2,
            k: 2,
            expected: "99".to_string(),
        },
        TestData {
            s: "102".to_string(),
            n: 3,
            k: 2,
            expected: "909".to_string(),
        },
        TestData {
            s: "12321".to_string(),
            n: 5,
            k: 1,
            expected: "12921".to_string(),
        },
        TestData {
            s: "11111111".to_string(),
            n: 8,
            k: 4,
            expected: "99111199".to_string(),
        },
        TestData {
            s: "11111111".to_string(),
            n: 8,
            k: 5,
            expected: "99111199".to_string(),
        },
        TestData {
            s: "1111111".to_string(),
            n: 7,
            k: 5,
            expected: "9919199".to_string(),
        },
        TestData {
            s: "11111111".to_string(),
            n: 8,
            k: 1,
            expected: "11111111".to_string(),
        },
        TestData {
            s: "1111111".to_string(),
            n: 7,
            k: 1,
            expected: "1119111".to_string(),
        },
        TestData {
            s: extremely_long_given,
            n: 77543,
            k: 58343,
            expected: extremely_long_expected,
        },
    ] {
        let actual = highest_value_palindrome(data.s, data.n, data.k);
        assert_eq!(actual, data.expected);
    }
}

#[allow(dead_code)]
fn read_extremely_long_test_data() -> (String, String) {
    let path =
        Path::new("./src/hacker_rank/algorithms/strings/test_data/highest_value_palindrome.csv");
    let mut lines = read_lines(path).unwrap();
    (
        lines.next().unwrap().unwrap(),
        lines.next().unwrap().unwrap(),
    )
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
