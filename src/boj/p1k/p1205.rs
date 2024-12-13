use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1205(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, score, p) = read_values_as!(read_line(reader), i32, i32, i32);

    // 랭킹 리스트가 비어있는 경우
    if n == 0 {
        writeln!(writer, "{}", if p > 0 { 1 } else { -1 }).unwrap();
        return;
    }

    let scores: Vec<i32> = {
        let line = read_line(reader);
        line.split_whitespace()
            .map(|x| x.parse::<i32>().expect("parse error"))
            .collect()
    };

    let last_score = *scores.last().expect("scores should not be empty");

    // 랭킹이 가득 찼고 새로운 점수가 마지막 점수보다 크지 않으면 진입 불가
    if n == p && score <= last_score {
        writeln!(writer, "-1").expect("writeln! should work");
        return;
    }

    let higher_count = scores.iter().filter(|&&s| s > score).count();
    let rank = (higher_count as i32) + 1;

    // 순위가 p보다 크면 리스트 진입 불가
    writeln!(writer, "{}", if rank > p { -1 } else { rank }).expect("writeln! should work");
}

// https://www.acmicpc.net/problem/1205
// 등수 구하기
#[test]
fn test_solve1205() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 90 10
100 90 80"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "10 1 10
10 9 8 7 6 5 4 3 2 1"
                .to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "10 1 10
10 9 8 7 6 5 4 3 3 0"
                .to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "0 0 50".to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1205(&mut reader, &mut writer);

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
