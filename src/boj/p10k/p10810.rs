use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10810(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut baskets = vec![0; n];

    for _ in 0..m {
        let s = read_line(reader);

        let (i, j, k) = {
            let mut iter = s.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        for idx in baskets.iter_mut().take(j).skip(i - 1) {
            *idx = k;
        }
    }

    let output = baskets
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    write!(writer, "{}", output).unwrap();
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
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
