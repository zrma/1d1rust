use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::cmp::Ordering;
use std::io::{BufRead, Write};

#[allow(dead_code)]
// noinspection SpellCheckingInspection
fn solve10214(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    const GAME_COUNT: usize = 9;
    for _ in 0..num_cases {
        let results: Vec<(i32, i32)> = (0..GAME_COUNT)
            .map(|_| {
                let (y, k) = read_values_as!(read_line(reader), i32, i32);
                (y, k)
            })
            .collect();

        let yonsei_score: i32 = results.iter().map(|&(y, _)| y).sum();
        let korea_score: i32 = results.iter().map(|&(_, k)| k).sum();

        let ans = match yonsei_score.cmp(&korea_score) {
            Ordering::Greater => "Yonsei",
            Ordering::Less => "Korea",
            Ordering::Equal => "Draw",
        };

        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/10214
// noinspection SpellCheckingInspection
// Baseball
#[test]
fn test_solve10214() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
1 0
0 0
0 0
0 0
0 0
0 0
0 0
0 0
0 0"
            .to_string(),
            want: "Yonsei".to_string(),
        },
        TestData {
            s: "2
1 0
0 0
0 0
0 0
0 1
0 1
0 0
0 0
0 0
1 0
0 0
0 0
0 0
0 0
0 0
0 0
0 0
0 1"
            .to_string(),
            want: "Korea
Draw"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10214(&mut reader, &mut writer);

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
