use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4504(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut answers = Vec::with_capacity(n);
    loop {
        let m: i32 = read_value(read_line(reader));
        if m == 0 {
            break;
        }
        answers.push(if m % n as i32 == 0 {
            format!("{} is a multiple of {}.", m, n)
        } else {
            format!("{} is NOT a multiple of {}.", m, n)
        });
    }

    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/4504
// 배수 찾기
#[test]
fn test_solve4504() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
1
7
99
321
777
0"
            .to_string(),
            want: "1 is NOT a multiple of 3.
7 is NOT a multiple of 3.
99 is a multiple of 3.
321 is a multiple of 3.
777 is a multiple of 3."
                .to_string(),
        },
        TestData {
            s: "1
1
2
0"
            .to_string(),
            want: "1 is a multiple of 1.
2 is a multiple of 1."
                .to_string(),
        },
        TestData {
            s: "2
1
2
0"
            .to_string(),
            want: "1 is NOT a multiple of 2.
2 is a multiple of 2."
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4504(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
