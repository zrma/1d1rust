use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10813(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);

    let mut balls = (1..=n).collect::<Vec<_>>();

    for _ in 0..m {
        let (i, j) = read_values_as!(read_line(reader), usize, usize);

        balls.swap(i - 1, j - 1);
    }

    let output = balls
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    writeln!(writer, "{}", output).unwrap();
}

// https://www.acmicpc.net/problem/10813
// 공 바꾸기
#[test]
fn test_solve10813() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "5 4
1 2
3 4
1 4
2 2"
        .to_string(),
        want: "3 1 4 2 5".to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10813(&mut reader, &mut writer);

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
