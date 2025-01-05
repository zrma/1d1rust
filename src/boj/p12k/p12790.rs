use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12790(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: i64 = read_value(read_line(reader));

    for _ in 0..num_cases {
        let (base_hp, base_mp, base_atk, base_def, equip_hp, equip_mp, equip_atk, equip_def) =
            read_values_as!(read_line(reader), i64, i64, i64, i64, i64, i64, i64, i64);

        let power = (base_hp + equip_hp).max(1)
            + (base_mp + equip_mp).max(1) * 5
            + (base_atk + equip_atk).max(0) * 2
            + (base_def + equip_def) * 2;

        writeln!(writer, "{}", power).unwrap();
    }
}

// https://www.acmicpc.net/problem/12790
// Mini Fantasy War
#[test]
fn test_solve12790() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
100 100 100 100 0 0 0 0
10 20 30 40 40 30 20 10
100 100 100 100 -200 0 400 400"
                .to_string(),
            want: "1000
500
2501"
                .to_string(),
        },
        TestData {
            s: "1
1 1 1 1 0 -1 -1 -1"
                .to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "1
100 100 100 100 0 -100 0 0"
                .to_string(),
            want: "505".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve12790(&mut reader, &mut writer);

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
