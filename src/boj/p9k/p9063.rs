use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9063(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_line(reader).parse().unwrap();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for _ in 0..n {
        let nums: Vec<i32> = read_line(reader)
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let x = nums[0];
        let y = nums[1];
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let res = (max_x - min_x) * (max_y - min_y);
    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/9063
// 대지
#[test]
fn test_solve9063() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
20 24
40 21
10 12"
                .to_string(),
            want: "360".to_string(),
        },
        TestData {
            s: "1
15 13"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "4
2 1
3 2
5 2
3 4"
            .to_string(),
            want: "9".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9063(&mut reader, &mut writer);

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
