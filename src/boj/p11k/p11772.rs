use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11772(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let ans = (0..n).fold(0, |acc, _| {
        let p: u64 = read_value(read_line(reader));
        let power = p % 10;
        let base = p / 10;
        acc + base.pow(power as u32)
    });

    writeln!(writer, "{}", ans).expect("write should succeed");
}

// https://www.acmicpc.net/problem/11772
// POT
#[test]
fn test_solve11772() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
212
1253"
                .to_string(),
            want: "1953566".to_string(),
        },
        TestData {
            s: "5
23
17
43
52
22"
            .to_string(),
            want: "102".to_string(),
        },
        TestData {
            s: "3
213
102
45"
            .to_string(),
            want: "10385".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11772(&mut reader, &mut writer);

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
