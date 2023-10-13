use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13163(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<i64>().unwrap();

    for _ in 0..n {
        let s = read_line(reader);

        let mut iter = s.split_whitespace();
        iter.next();
        let mut res = "god".to_string();
        for word in iter {
            res.push_str(word);
        }
        writeln!(writer, "{}", res).unwrap();
    }
}

// https://www.acmicpc.net/problem/13163
// 닉네임에 갓 붙이기
// noinspection SpellCheckingInspection
#[test]
fn test_solve13163() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
baek joon
koo sa ga
ac ka
yu ka ri ko
ke sa ki yo"
                .to_string(),
            want: "godjoon
godsaga
godka
godkariko
godsakiyo
"
            .to_string(),
        },
        TestData {
            s: "4
baek joon
koo sa ga
ac ka
yu ka ri ko"
                .to_string(),
            want: "godjoon
godsaga
godka
godkariko
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13163(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
