use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17103(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut is_prime = vec![true; 1_000_001];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=1_000_000 {
        if is_prime[i] {
            for j in (i * 2..=1_000_000).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let mut primes = vec![];
    for (i, it) in is_prime.iter().enumerate().take(1_000_000 + 1).skip(2) {
        if *it {
            primes.push(i);
        }
    }

    for _ in 0..n {
        let x: usize = read_value(read_line(reader));
        let mut count = 0;
        for &p in &primes {
            if p > x / 2 {
                break;
            }
            if is_prime[x - p] {
                count += 1;
            }
        }
        writeln!(writer, "{}", count).unwrap();
    }
}

// https://www.acmicpc.net/problem/17103
// 골드바흐 파티션
#[test]
fn test_solve17103() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "5
6
8
10
12
100"
        .to_string(),
        want: "1
1
2
1
6
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17103(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
