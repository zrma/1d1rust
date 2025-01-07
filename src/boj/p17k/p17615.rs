use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17615(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let s = read_line(reader);
    let balls: Vec<char> = s.chars().collect();

    let red_left = moves_to_group_left(&balls, 'R');
    let red_right = moves_to_group_right(&balls, 'R');
    let blue_left = moves_to_group_left(&balls, 'B');
    let blue_right = moves_to_group_right(&balls, 'B');

    let min_moves = [red_left, red_right, blue_left, blue_right]
        .iter()
        .min()
        .copied()
        .unwrap_or(n);

    writeln!(writer, "{}", min_moves).unwrap();
}

/// target 색을 왼쪽으로 모으기
fn moves_to_group_left(balls: &[char], target: char) -> usize {
    moves_to_group(balls.iter(), target)
}

/// target 색을 오른쪽으로 모으기
fn moves_to_group_right(balls: &[char], target: char) -> usize {
    moves_to_group(balls.iter().rev(), target)
}

/// 내부 공통로직: &char를 순회하면서 target 색 공의 이동 횟수를 계산
fn moves_to_group<'a, I>(iter: I, target: char) -> usize
where
    I: Iterator<Item = &'a char>,
{
    let mut count = 0;
    let mut seen_other = false;

    for &ball in iter {
        if ball != target {
            seen_other = true;
        } else if seen_other {
            count += 1;
        }
    }

    count
}

// https://www.acmicpc.net/problem/17615
// 볼 모으기
#[test]
fn test_solve17615() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9
RBBBRBRRR"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "8
BBRBBBBR"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "10
RRRRRRRRRR"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "6
RRBBRR"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17615(&mut reader, &mut writer);

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
