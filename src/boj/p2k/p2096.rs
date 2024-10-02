use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2096(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut max_dp = [0; 3];
    let mut min_dp = [0; 3];
    let mut line = String::new();

    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).unwrap();
        update_dp(&mut max_dp, &mut min_dp, &line);
    }

    let max_score = *max_dp.iter().max().unwrap();
    let min_score = *min_dp.iter().min().unwrap();

    write!(writer, "{} {}", max_score, min_score).expect("Failed to write");
}

fn update_dp(max_dp: &mut [i32; 3], min_dp: &mut [i32; 3], line: &str) {
    let (a, b, c) = read_values_as!(line, i32, i32, i32);

    let new_max = [
        a + max_dp[0].max(max_dp[1]),
        b + max_dp[0].max(max_dp[1]).max(max_dp[2]),
        c + max_dp[1].max(max_dp[2]),
    ];
    let new_min = [
        a + min_dp[0].min(min_dp[1]),
        b + min_dp[0].min(min_dp[1]).min(min_dp[2]),
        c + min_dp[1].min(min_dp[2]),
    ];

    *max_dp = new_max;
    *min_dp = new_min;
}

// https://www.acmicpc.net/problem/2096
// 내려가기
#[test]
fn test_solve2096() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
1 2 3
4 5 6
4 9 0"
                .to_string(),
            want: "18 6".to_string(),
        },
        TestData {
            s: "3
0 0 0
0 0 0
0 0 0"
                .to_string(),
            want: "0 0".to_string(),
        },
        TestData {
            s: "4
1 1 1
1 1 1
1 1 1
1 1 1"
                .to_string(),
            want: "4 4".to_string(),
        },
        TestData {
            s: "4
3 1 2
2 1 3
1 3 2
3 1 2"
                .to_string(),
            want: "11 4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2096(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
