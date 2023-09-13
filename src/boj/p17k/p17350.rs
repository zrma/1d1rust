use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17350(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    let mut found = false;
    for _ in 0..n {
        let s = read_line(reader);
        if s == "anj" {
            found = true;
        }
    }

    if found {
        write!(writer, "뭐야;").unwrap();
    } else {
        write!(writer, "뭐야?").unwrap();
    }
}

// https://www.acmicpc.net/problem/17350
// 2루수 이름이 뭐야
// noinspection SpellCheckingInspection
#[test]
fn test_solve17350() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "3
snrn
anj
ahffk"
                .to_string(),
            want: "뭐야;".to_string(),
        },
        TestData {
            s: "4
aptl
molamolamolamola
dlqmfkglahqlcl
QWERTOP"
                .to_string(),
            want: "뭐야?".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17350(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
