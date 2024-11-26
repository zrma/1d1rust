use crate::utils::functions::char_to_index;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12813(reader: &mut impl BufRead, writer: &mut impl Write) {
    let a = read_line(reader);
    let b = read_line(reader);

    let a = a.chars().map(char_to_index::<u8>).collect::<Vec<_>>();
    let b = b.chars().map(char_to_index::<u8>).collect::<Vec<_>>();

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
    for (i, data) in [
        TestData {
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
        },
        TestData {
            s: "0001011000
0000101110"
                .to_string(),
            want: "0000001000
0001111110
0001110110
1110100111
1111010001
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve12813(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
