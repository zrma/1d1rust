use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20353(reader: &mut impl BufRead, writer: &mut impl Write) {
    let area = read_line(reader).parse::<f64>().unwrap();
    let res = area.sqrt() * 4.0;
    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/20353
// Atrium
#[test]
fn test_solve20353() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "64".to_string(),
            want: "32.0".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "4.0".to_string(),
        },
        TestData {
            s: "1234".to_string(),
            want: "140.51334456".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "40".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20353(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap().parse::<f64>().unwrap();
        let want = data.want.parse::<f64>().unwrap();

        assert!((got - want).abs() < 1e-6, "case {}", i);
    }
}
