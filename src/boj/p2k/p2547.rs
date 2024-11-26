use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2547(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let mut lines = reader.lines();

    for _ in 0..num_cases {
        let n = read_next_non_empty_line(&mut lines)
            .expect("next line should exist")
            .parse::<u64>()
            .expect("n should be parseable as u64");

        let n_u128 = n as u128;
        let mut total_mod_n = 0u128;

        for _ in 0..n {
            let candies = read_next_non_empty_line(&mut lines)
                .expect("next line should exist")
                .parse::<u128>()
                .expect("candies should be parseable as u128");

            total_mod_n = (total_mod_n + candies % n_u128) % n_u128;
        }

        writeln!(writer, "{}", if total_mod_n == 0 { "YES" } else { "NO" }).unwrap();
    }
}

fn read_next_non_empty_line(
    lines: &mut impl Iterator<Item = Result<String, std::io::Error>>,
) -> Option<String> {
    lines.find_map(|line| line.ok().filter(|l| !l.trim().is_empty()))
}

// https://www.acmicpc.net/problem/2547
// 사탕 선생 고창영
#[test]
fn test_solve2547() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2

5
5
2
7
3
8

6
7
11
2
7
3
4"
            .to_string(),
            want: "YES
NO
"
            .to_string(),
        },
        TestData {
            s: "1

3
3
6
9"
            .to_string(),
            want: "YES
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2547(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
