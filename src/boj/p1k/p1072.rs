use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1072(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (num_games, num_wins) = read_values_as!(read_line(reader), i64, i64);
    let win_ratio = (num_wins * 100) / num_games;
    if win_ratio >= 99 {
        write!(writer, "-1").expect("write! should work");
        return;
    }
    let mut lower_bound = 1;
    let mut upper_bound = num_games * 10000;
    while lower_bound < upper_bound {
        let mid = (lower_bound + upper_bound) / 2;
        let new_ratio = ((num_wins + mid) * 100) / (num_games + mid);
        if new_ratio > win_ratio {
            upper_bound = mid;
        } else {
            lower_bound = mid + 1;
        }
    }
    write!(writer, "{}", lower_bound).expect("write! should work");
}

// https://www.acmicpc.net/problem/1072
// 게임
#[test]
fn test_solve1072() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10 8".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "100 80".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "47 47".to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "99000 0".to_string(),
            want: "1000".to_string(),
        },
        TestData {
            s: "1000000000 470000000".to_string(),
            want: "19230770".to_string(),
        },
        TestData {
            s: "1000000000 1".to_string(),
            want: "10101010".to_string(),
        },
        TestData {
            s: "1000000000 989999999".to_string(),
            want: "100".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1072(&mut reader, &mut writer);

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
