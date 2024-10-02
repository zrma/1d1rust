use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4766(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut res = vec![];
    let mut prev: f32 = read_value(read_line(reader));
    loop {
        let curr = read_value(read_line(reader));
        if curr == 999.0 {
            break;
        }

        res.push(format!("{:.2}", curr - prev));
        prev = curr;
    }

    write!(writer, "{}", res.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/4766
// 일반 화학 실험
#[test]
fn test_solve4766() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10.0
12.05
30.25
20
999"
            .to_string(),
            want: "2.05
18.20
-10.25"
                .to_string(),
        },
        TestData {
            s: "0.0
1
2.5
0.0
999"
            .to_string(),
            want: "1.00
1.50
-2.50"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4766(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
