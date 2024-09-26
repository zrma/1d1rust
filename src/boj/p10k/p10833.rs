use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10833(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_students: usize = read_value(read_line(reader));
    let ans: i32 = (0..num_students)
        .map(|_| {
            let (students, apples) = read_values_as!(read_line(reader), i32, i32);
            apples % students
        })
        .sum();

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/10833
// 사과
#[test]
fn test_solve10833() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
24 52
13 22
5 53
23 10
7 70"
                .to_string(),
            want: "26".to_string(),
        },
        TestData {
            s: "3
10 20
5 5
1 13"
                .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10833(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
