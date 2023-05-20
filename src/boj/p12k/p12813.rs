use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12813(reader: &mut impl BufRead, writer: &mut impl Write) {
    let a = read_line(reader);
    let b = read_line(reader);

    let a = a
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect::<Vec<_>>();
    let b = b
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect::<Vec<_>>();

    let res = [
        a.iter().zip(&b).map(|(&a, &b)| a & b).collect::<Vec<_>>(),
        a.iter().zip(&b).map(|(&a, &b)| a | b).collect::<Vec<_>>(),
        a.iter().zip(&b).map(|(&a, &b)| a ^ b).collect::<Vec<_>>(),
        a.iter().map(|&a| a ^ 1).collect::<Vec<_>>(),
        b.iter().map(|&b| b ^ 1).collect::<Vec<_>>(),
    ];

    for r in res {
        writeln!(
            writer,
            "{}",
            r.iter().map(|&b| b.to_string()).collect::<String>()
        )
        .unwrap();
    }
}

// https://www.acmicpc.net/problem/12813
// 이진수 연산
#[test]
fn test_solve12813() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
        s: "0001011000
0000101111"
            .to_string(),
        want: "0000001000
0001111111
0001110111
1110100111
1111010000
"
        .to_string(),
    }]
    .into_iter()
    .enumerate()
    {
        use std::io::Cursor;
        let mut reader = Cursor::new(data.s);
        let mut writer = Cursor::new(Vec::new());
        solve12813(&mut reader, &mut writer);

        let got = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}
