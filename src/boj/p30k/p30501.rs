use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve30501(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_names = read_value(read_line(reader));
    let names = (0..num_of_names)
        .map(|_| read_line(reader))
        .collect::<Vec<_>>();

    let killer = names.iter().find(|name| name.contains('S')).unwrap();

    writeln!(writer, "{}", killer).unwrap();
}

// https://www.acmicpc.net/problem/30501
// noinspection SpellCheckingInspection
// 관공... 어찌하여 목만 오셨소...
#[test]
fn test_solve30501() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
ZHOUYU
SUNQUAN
ZOZO"
                .to_string(),
            want: "SUNQUAN".to_string(),
        },
        TestData {
            s: "1
SUSEMI"
                .to_string(),
            want: "SUSEMI".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve30501(&mut reader, &mut writer);

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
