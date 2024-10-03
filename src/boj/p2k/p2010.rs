use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2010(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_multi_plugs: usize = read_value(read_line(reader));
    let mut total_ports = 0;

    for _ in 0..num_multi_plugs {
        total_ports += read_value::<usize>(read_line(reader));
    }

    let available_ports = total_ports - num_multi_plugs + 1;
    write!(writer, "{}", available_ports).expect("write! should work");
}

// https://www.acmicpc.net/problem/2010
// noinspection SpellCheckingInspection
// 플러그
#[test]
fn test_solve2010() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
1
1
1"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2
5
8"
            .to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "3
3
2
1"
            .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2010(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
