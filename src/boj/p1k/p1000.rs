use crate::utils::parse;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1000(reader: &mut impl BufRead, writer: &mut impl Write) {
    let lines = reader.lines();

    let arr = parse::str_to_arr(lines);
    let a = arr[0];
    let b = arr[1];

    let res = a + b;
    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/1000
#[test]
fn test_solve1000() {
    struct TestData {
        s: String,
        want: String,
    }
    for data in std::vec![
        TestData {
            s: "1 2".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "-1 -2".to_string(),
            want: "-3".to_string(),
        },
        TestData {
            s: "-1 2".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1 -2".to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "0 0".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1 9".to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "9 1".to_string(),
            want: "10".to_string(),
        },
    ] {
        use std::io::Cursor;
        let mut reader = Cursor::new(data.s);
        let mut writer = Cursor::new(Vec::new());
        solve1000(&mut reader, &mut writer);

        let got = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(data.want, got);
    }
}
