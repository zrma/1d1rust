use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18110(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    if n == 0 {
        writeln!(writer, "0").unwrap();
        return;
    }

    let round_target = (n as f32 * 0.15).round() as usize;
    let mut nums: Vec<f32> = (0..n)
        .map(|_| read_value(read_line(reader)))
        .collect::<Vec<_>>();

    nums.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let sum: f32 = nums
        .iter()
        .skip(round_target)
        .take(n - 2 * round_target)
        .sum();
    let count = n - 2 * round_target;

    let res = if count > 0 {
        (sum / count as f32).round() as i32
    } else {
        0
    };

    writeln!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/18110
// solved.ac
#[test]
fn test_solve18110() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1
5
5
7
8"
            .to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "10
1
13
12
15
3
16
13
12
14
15"
            .to_string(),
            want: "13".to_string(),
        },
        TestData {
            s: "0".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18110(&mut reader, &mut writer);

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
