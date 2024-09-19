use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2712(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_value(read_line(reader));
    let ans = (0..t)
        .map(|_| {
            let (value, unit) = read_values_as!(read_line(reader), f64, String);
            convert_unit(value, &unit)
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).unwrap();
}

fn convert_unit(value: f64, unit: &str) -> String {
    let (converted_value, new_unit) = match unit {
        "kg" => (value * 2.2046, "lb"),
        "lb" => (value * 0.4536, "kg"),
        "l" => (value * 0.2642, "g"),
        "g" => (value * 3.7854, "l"),
        _ => panic!("Invalid unit"),
    };
    format!("{:.4} {}", converted_value, new_unit)
}

// https://www.acmicpc.net/problem/2712
// 미국 스타일
#[test]
fn test_solve2712() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1 kg
2 l
7 lb
3.5 g
0 l"
            .to_string(),
            want: "2.2046 lb
0.5284 g
3.1752 kg
13.2489 l
0.0000 g"
                .to_string(),
        },
        TestData {
            s: "4
2.2046 lb
0.4536 kg
0.2642 g
3.7854 l"
                .to_string(),
            want: "1.0000 kg
1.0000 lb
1.0001 l
1.0001 g"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2712(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
