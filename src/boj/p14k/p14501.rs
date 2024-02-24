use crate::read_values;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14501(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut schedules = vec![(0, 0); n + 1];

    for schedule in schedules.iter_mut().skip(1) {
        *schedule = read_values!(read_line(reader), usize, usize);
    }

    let mut dp = vec![0; n + 2];
    for i in (1..=n).rev() {
        let (duration, profit) = schedules[i];
        let completion_day = i + duration;

        dp[i] = if completion_day <= n + 1 {
            std::cmp::max(dp[i + 1], profit + dp[completion_day])
        } else {
            dp[i + 1]
        };
    }

    write!(writer, "{}", dp[1]).unwrap();
}

// https://www.acmicpc.net/problem/14501
// 퇴사
#[test]
fn test_solve14501() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
3 10
5 20
1 10
1 20
2 15
4 40
2 200"
                .to_string(),
            want: "45".to_string(),
        },
        TestData {
            s: "10
1 1
1 2
1 3
1 4
1 5
1 6
1 7
1 8
1 9
1 10"
                .to_string(),
            want: "55".to_string(),
        },
        TestData {
            s: "10
5 10
5 9
5 8
5 7
5 6
5 10
5 9
5 8
5 7
5 6"
            .to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "10
5 50
4 40
3 30
2 20
1 10
1 10
2 20
3 30
4 40
5 50"
                .to_string(),
            want: "90".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14501(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
