use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20125(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let line = read_line(reader);
        grid.push(line.chars().collect::<Vec<_>>());
    }

    // 심장 위치 찾기 (머리 바로 아래)
    let mut heart_x = 0;
    let mut heart_y = 0;
    'outer: for (i, row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '*' {
                heart_x = i + 1;
                heart_y = j;
                break 'outer;
            }
        }
    }

    // 왼쪽 팔 길이
    let mut left_arm = 0;
    for j in (0..heart_y).rev() {
        if grid[heart_x][j] == '*' {
            left_arm += 1;
        } else {
            break;
        }
    }

    // 오른쪽 팔 길이
    let mut right_arm = 0;
    for j in (heart_y + 1)..n {
        if grid[heart_x][j] == '*' {
            right_arm += 1;
        } else {
            break;
        }
    }

    // 허리 길이
    let mut waist = 0;
    let mut waist_end = heart_x + 1;
    for (i, row) in grid.iter().enumerate().skip(heart_x + 1) {
        if row[heart_y] == '*' {
            waist += 1;
            waist_end = i;
        } else {
            break;
        }
    }

    // 왼쪽 다리 길이
    let mut left_leg = 0;
    for row in grid.iter().skip(waist_end + 1) {
        if row[heart_y - 1] == '*' {
            left_leg += 1;
        } else {
            break;
        }
    }

    // 오른쪽 다리 길이
    let mut right_leg = 0;
    for row in grid.iter().skip(waist_end + 1) {
        if row[heart_y + 1] == '*' {
            right_leg += 1;
        } else {
            break;
        }
    }

    writeln!(writer, "{} {}", heart_x + 1, heart_y + 1).unwrap();
    writeln!(
        writer,
        "{} {} {} {} {}",
        left_arm, right_arm, waist, left_leg, right_leg
    )
    .unwrap();
}

// https://www.acmicpc.net/problem/20125
// 쿠키의 신체 측정
#[test]
fn test_solve20125() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
_____
__*__
_***_
__*__
_*_*_"
                .to_string(),
            want: "3 3
1 1 1 1 1"
                .to_string(),
        },
        TestData {
            s: "10
__________
_____*____
__******__
_____*____
_____*____
_____*____
____*_*___
____*_____
____*_____
____*_____"
                .to_string(),
            want: "3 6
3 2 3 4 1"
                .to_string(),
        },
        TestData {
            s: "9
____*____
*********
____*____
____*____
____*____
___*_*___
___*_*___
___*_*___
___*_*___"
                .to_string(),
            want: "2 5
4 4 3 4 4"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20125(&mut reader, &mut writer);

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
