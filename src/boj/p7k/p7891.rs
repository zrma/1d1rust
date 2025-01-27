use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve7891(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: i32 = read_value(read_line(reader));

    for _ in 0..t {
        let (x, y): (i64, i64) = read_values_as!(read_line(reader), i64, i64);
        writeln!(writer, "{}", x + y).unwrap();
    }
}

// https://www.acmicpc.net/problem/7891
// Can you add this?
#[test]
fn test_solve7891() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "4
1 1
2 2
3 3
-1 -1"
                .to_string(),
            want: "2
4
6
-2"
            .to_string(),
        },
        TestCase {
            s: "2
1000000000 1000000000
-1000000000 -1000000000"
                .to_string(),
            want: "2000000000
-2000000000"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve7891(&mut reader, &mut writer);

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
