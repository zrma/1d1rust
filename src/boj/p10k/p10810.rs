use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10810(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);

    let mut baskets = vec![0; n];

    for _ in 0..m {
        let (i, j, k) = read_values_as!(read_line(reader), usize, usize, usize);

        for idx in baskets.iter_mut().take(j).skip(i - 1) {
            *idx = k;
        }
    }

    let output = baskets
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    writeln!(writer, "{}", output).unwrap();
}

// https://www.acmicpc.net/problem/10810
// 공 넣기
#[test]
fn test_solve10810() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "5 4
1 2 3
3 4 4
1 4 1
2 2 2"
            .to_string(),
        want: "1 2 1 1 0".to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10810(&mut reader, &mut writer);

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
