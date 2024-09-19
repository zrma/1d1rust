use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9506(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let n: i32 = read_line(reader).parse().unwrap();
        if n == -1 {
            break;
        }

        let mut divisors = vec![];
        let mut sum = 0;
        for i in 1..n {
            if n % i == 0 {
                divisors.push(i);
                sum += i;
            }
        }

        let res = if sum == n {
            format!(
                "{} = {}",
                n,
                divisors
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" + ")
            )
        } else {
            format!("{} is NOT perfect.", n)
        };
        writeln!(writer, "{}", res).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/9506
// 약수들의 합
#[test]
fn test_solve9506() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "6
12
28
-1"
        .to_string(),
        want: "6 = 1 + 2 + 3
12 is NOT perfect.
28 = 1 + 2 + 4 + 7 + 14
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9506(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
