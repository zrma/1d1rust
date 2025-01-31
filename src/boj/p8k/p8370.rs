use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8370(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n1, k1, n2, k2): (i32, i32, i32, i32) =
        read_values_as!(read_line(reader), i32, i32, i32, i32);
    let total = n1 * k1 + n2 * k2;
    writeln!(writer, "{}", total).unwrap();
}

// https://www.acmicpc.net/problem/8370
// Plane
#[test]
fn test_solve8370() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "2 5 3 20".to_string(),
            want: "70".to_string(),
        },
        TestCase {
            s: "1 1 1 1".to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8370(&mut reader, &mut writer);

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
