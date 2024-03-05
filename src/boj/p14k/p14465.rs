use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14465(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, k, b) = read_values_as!(read_line(reader), usize, usize, usize);
    let mut broken = vec![false; n + 1];
    for _ in 0..b {
        let x: usize = read_value(read_line(reader));
        broken[x] = true;
    }

    let mut current_broken = (1..=k).filter(|&i| broken[i]).count();
    let mut min_broken = current_broken;
    for i in k + 1..=n {
        if broken[i] {
            current_broken += 1;
        }
        if broken[i - k] {
            current_broken -= 1;
        }
        min_broken = min_broken.min(current_broken);
    }

    write!(writer, "{}", min_broken).expect("Failed to write");
}

// https://www.acmicpc.net/problem/14465
// 소가 길을 건너간 이유 5
#[test]
fn test_solve14465() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10 6 5
2
10
1
5
9"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "10 2 5
2
10
1
5
9"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "10 3 5
2
10
1
5
9"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "10 4 5
2
10
1
5
9"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "10 10 5
2
10
1
5
9"
            .to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14465(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
