use crate::read_values;
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve22233(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let (n, m) = read_values!(&line, usize, usize);

    let mut keywords = (0..n)
        .map(|_| {
            line.clear();
            reader.read_line(&mut line).unwrap();
            line.trim().to_owned()
        })
        .collect::<HashSet<_>>();

    let mut results = Vec::with_capacity(m);
    for _ in 0..m {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let query = line
            .split(',')
            .map(str::trim)
            .map(ToOwned::to_owned)
            .collect::<HashSet<_>>();

        for q in &query {
            keywords.remove(q);
        }
        results.push(format!("{}", keywords.len()));
    }

    write!(writer, "{}", results.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/22233
// noinspection SpellCheckingInspection
// 가희와 키워드
#[test]
fn test_solve22233() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 2
map
set
dijkstra
floyd
os
map,dijkstra
map,floyd"
                .to_string(),
            want: "3
2"
            .to_string(),
        },
        TestData {
            s: "2 2
gt26cw
1211train
kiwoom,lottegiant
kbo"
            .to_string(),
            want: "2
2"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve22233(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
