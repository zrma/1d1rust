use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6768(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let ans = if n < 4 {
        0
    } else {
        let n = n - 1;
        // nC3 = (n*(n-1)*(n-2))/6
        (n * (n - 1) * (n - 2)) / 6
    };

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/6768
// Don’t pass me the ball!
#[test]
fn test_solve6768() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "90".to_string(),
            want: "113564".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6768(&mut reader, &mut writer);

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
