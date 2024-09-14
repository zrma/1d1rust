use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1837(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (p_str, k) = read_values_as!(read_line(reader), String, u32);
    let k = k as usize;

    let mut is_prime = vec![true; k];
    is_prime[0] = false;
    if k > 1 {
        is_prime[1] = false;
    }

    for i in 2..k {
        if is_prime[i] {
            (i * 2..k).step_by(i).for_each(|j| is_prime[j] = false);
        }
    }

    if let Some(bad_prime) = (2..k).find(|&i| is_prime[i] && compute_mod(&p_str, i as u32) == 0) {
        write!(writer, "BAD {}", bad_prime).unwrap();
    } else {
        write!(writer, "GOOD").unwrap();
    }
}

fn compute_mod(p_str: &str, m: u32) -> u32 {
    p_str
        .chars()
        .fold(0, |acc, c| (acc * 10 + (c as u8 - b'0') as u32) % m)
}

// https://www.acmicpc.net/problem/1837
// 암호제작
#[test]
fn test_solve1837() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "143 10".to_string(),
            want: "GOOD".to_string(),
        },
        TestData {
            s: "77 12".to_string(),
            want: "BAD 7".to_string(),
        },
        TestData {
            s: "50419560124 10".to_string(),
            want: "BAD 2".to_string(),
        },
        TestData {
            s: "222222222222222222222222222 100000".to_string(),
            want: "BAD 2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1837(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
