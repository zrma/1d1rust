use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27159(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let arr: Vec<usize> = read_n_values(reader, n);

    let mut sum = 0;
    let mut prev = None;

    for &x in &arr {
        if let Some(p) = prev {
            if x != p + 1 {
                sum += x;
            }
        } else {
            sum += x;
        }
        prev = Some(x);
    }

    write!(writer, "{}", sum).expect("write! should work");
}

// https://www.acmicpc.net/problem/27159
// 노 땡스!
#[test]
fn test_solve27159() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10
6 7 10 12 13 14 15 20 21 22"
                .to_string(),
            want: "48".to_string(),
        },
        TestData {
            s: "1
33"
            .to_string(),
            want: "33".to_string(),
        },
        TestData {
            s: "2
33 34"
                .to_string(),
            want: "33".to_string(),
        },
        TestData {
            s: "2
33 35"
                .to_string(),
            want: "68".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27159(&mut reader, &mut writer);

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
