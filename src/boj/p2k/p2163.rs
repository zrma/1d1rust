use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2163(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);

    // 가로로 n 크기이므로 n - 1번 자르기. (n조각)
    // 세로로 m 크기이므로 m - 1번 자르기. (m조각) 이걸 n 조각 반복
    // (n - 1) + n * (m - 1) = n - 1 + n * m - n = n * m - 1
    write!(writer, "{}", n * m - 1).unwrap();
}

// https://www.acmicpc.net/problem/2163
// 초콜릿 자르기
#[test]
fn test_solve2163() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 2".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "5 5".to_string(),
            want: "24".to_string(),
        },
        TestData {
            s: "1 1".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1 2".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2 1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2 3".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "3 2".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "3 3".to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "3 4".to_string(),
            want: "11".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2163(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
