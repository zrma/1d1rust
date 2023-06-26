use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4134(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    for _ in 0..n {
        let mut x = read_line(reader).parse::<u64>().unwrap();
        if x <= 1 {
            writeln!(writer, "2").unwrap();
            continue;
        }
        while !is_prime(x) {
            x += 1;
        }
        writeln!(writer, "{}", x).unwrap();
    }
}

fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        _ if n % 2 == 0 => false,
        _ => {
            let mut d = n - 1;
            while d % 2 == 0 {
                d /= 2;
            }
            for &a in &[2, 7, 61] {
                if n == a {
                    return true;
                }
                if !miller_rabin(n, a, d) {
                    return false;
                }
            }
            true
        }
    }
}

fn miller_rabin(n: u64, a: u64, mut d: u64) -> bool {
    let mut x = pow_mod(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    while d != n - 1 {
        x = (x * x) % n;
        d *= 2;
        if x == n - 1 {
            return true;
        }
    }
    false
}

fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

// https://www.acmicpc.net/problem/4134
// 다음 소수
#[test]
fn test_solve4134() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
        s: "3
6
20
100"
        .to_string(),
        want: "7
23
101
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4134(reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
