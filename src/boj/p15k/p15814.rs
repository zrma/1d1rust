use crate::read_values;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15814(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let mut ans = s.chars().collect::<Vec<_>>();
    let n = read_value(read_line(reader));

    for _ in 0..n {
        let (i, j) = read_values!(read_line(reader), usize, usize);
        ans.swap(i, j);
    }

    write!(writer, "{}", ans.iter().collect::<String>()).unwrap();
}

// https://www.acmicpc.net/problem/15814
// noinspection SpellCheckingInspection
// 야바위 대장
#[test]
fn test_solve15814() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "Youngmaan-good
2
1 3
9 2"
            .to_string(),
            want: "Yn-ogmaanugood".to_string(),
        },
        TestData {
            s: "Hello World
1
0 10"
                .to_string(),
            want: "dello WorlH".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15814(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
