use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25175(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m, k) = read_values_as!(read_line(reader), i64, i64, i64);

    // 현재 사람은 항상 3모를 부름
    let curr_count: i64 = 3;

    // 부른 모 수 K에서 3을 뺀 값으로 반시계 방향 이동 계산
    let move_steps = k - curr_count;

    // 현재 차례인 사람을 0-based 인덱스로 변환 후, 이동 계산
    let next_player = (m - 1 + move_steps).rem_euclid(n) + 1;

    write!(writer, "{}", next_player).expect("Failed to write");
}

// https://www.acmicpc.net/problem/25175
// 두~~부 두부 두부
#[test]
fn test_solve25175() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "7 2 4".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "6 1 -1".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "3 1 -6".to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25175(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
