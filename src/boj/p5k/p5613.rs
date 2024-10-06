use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5613(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut ans = 0;
    let mut op = String::from("+");

    loop {
        let input = read_line(reader);
        if op == "=" {
            write!(writer, "{}", ans).expect("write! should work");
            break;
        }

        if let Ok(n) = input.parse::<i32>() {
            match op.as_str() {
                "+" => ans += n,
                "-" => ans -= n,
                "*" => ans *= n,
                "/" => ans /= n,
                _ => panic!("unexpected operator"),
            }
        } else {
            op = input;
        }
    }
}

// https://www.acmicpc.net/problem/5613
// 계산기 프로그램
#[test]
fn test_solve5613() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
+
1
="
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "10
-
21
*
5
="
            .to_string(),
            want: "-55".to_string(),
        },
        TestData {
            s: "100
/
3
*
3
="
            .to_string(),
            want: "99".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve5613(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
