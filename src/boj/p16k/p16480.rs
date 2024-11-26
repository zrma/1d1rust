use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16480(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (r0, r1) = read_values_as!(read_line(reader), i64, i64);

    write!(writer, "{}", r0 * (r0 - 2 * r1)).unwrap();
}

// https://www.acmicpc.net/problem/16480
// 외심과 내심은 사랑입니다
#[test]
fn test_solve16480() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4 1".to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "13 5".to_string(),
            want: "39".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16480(&mut reader, &mut writer);

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
