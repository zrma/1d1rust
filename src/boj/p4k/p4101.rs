use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4101(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let (n1, n2): (i32, i32) = read_values_as!(read_line(reader), i32, i32);
        if n1 == 0 && n2 == 0 {
            break;
        }

        writeln!(writer, "{}", if n1 > n2 { "Yes" } else { "No" }).unwrap();
    }
}

// https://www.acmicpc.net/problem/4101
// 크냐?
#[test]
fn test_solve4101() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "1 19
4 4
23 14
0 0"
            .to_string(),
            want: "No
No
Yes"
            .to_string(),
        },
        TestCase {
            s: "3 2
2 3
0 0"
            .to_string(),
            want: "Yes
No"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4101(&mut reader, &mut writer);

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
