use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5426(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value(read_line(reader));
    let mut answers = Vec::with_capacity(num_cases);

    for _ in 0..num_cases {
        let s = read_line(reader);
        let len = s.len();
        let size = (len as f64).sqrt() as usize;

        let mut board = vec![vec![' '; size]; size];
        for (i, c) in s.chars().enumerate() {
            let row = i / size;
            let col = i % size;
            board[size - 1 - col][row] = c;
        }

        let ans: String = board.iter().flat_map(|row| row.iter()).collect();
        answers.push(ans);
    }

    writeln!(writer, "{}", answers.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/5426
// noinspection SpellCheckingInspection
// 비밀 편지
#[test]
fn test_solve5426() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
RSTEEOTCP
eedARBtVrolsiesuAoReerles
EarSvyeqeBsuneMa"
                .to_string(),
            want: "TOPSECRET
RosesAreRedVioletsAreBlue
SquaresMayBeEven"
                .to_string(),
        },
        TestData {
            s: "1
a"
            .to_string(),
            want: "a".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5426(&mut reader, &mut writer);

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
