use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve26068(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();

    let mut ans = 0;
    for _ in 0..n {
        let t = read_line(reader);
        let t: i32 = t[2..].parse().unwrap();
        if t <= 90 {
            ans += 1;
        }
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/26068
// 치킨댄스를 추는 곰곰이를 본 임스 2
#[test]
fn test_solve26068() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
D-86
D-8
D-6"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "4
D-98
D-5
D-94
D-2"
            .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve26068(&mut reader, &mut writer);

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
