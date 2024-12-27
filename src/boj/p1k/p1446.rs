use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1446(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, d) = read_values_as!(read_line(reader), usize, usize);

    // 지름길 정보를 (start, end, dist) 형태로 저장
    let mut shortcuts = vec![];
    for _ in 0..n {
        let (start, end, dist) = read_values_as!(read_line(reader), usize, usize, usize);
        // end가 D보다 큰 경우와 지름길이 오히려 더 멀어지는 경우는 거름.
        if end <= d || start.abs_diff(end) < dist {
            shortcuts.push((start, end, dist));
        }
    }

    // DP 테이블 초기화: dp[i] = 0..i km 그대로 달렸을 때의 비용 = i
    let mut dp = (0..=d).collect::<Vec<usize>>();

    // 0부터 D까지 순회하며,
    //    1) i에서 i+1로 직접 고속도로를 탄 경우 비용 갱신
    //    2) i에서 시작하는 지름길을 탄 경우 비용 갱신
    for i in 0..d {
        // 고속도로로 1km 이동
        dp[i + 1] = dp[i + 1].min(dp[i] + 1);

        // i에서 시작하는 지름길을 확인
        for &(start, end, dist) in &shortcuts {
            if start == i {
                dp[end] = dp[end].min(dp[i] + dist);
            }
        }
    }

    writeln!(writer, "{}", dp[d]).unwrap();
}

// https://www.acmicpc.net/problem/1446
// 지름길
#[test]
fn test_solve1446() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 150
0 50 10
0 50 20
50 100 10
100 151 10
110 140 90"
                .to_string(),
            want: "70".to_string(),
        },
        TestData {
            s: "2 100
10 60 40
50 90 20"
                .to_string(),
            want: "80".to_string(),
        },
        TestData {
            s: "8 900
0 10 9
20 60 45
80 190 100
50 70 15
160 180 14
140 160 14
420 901 5
450 900 0"
                .to_string(),
            want: "432".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1446(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("valid utf8 string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
