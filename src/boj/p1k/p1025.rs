use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1025(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);
    let mut board = Vec::with_capacity(n);
    for _ in 0..n {
        let line = read_line(reader);
        board.push(line.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>());
    }

    let mut ans = -1;

    for i in 0..n {
        for j in 0..m {
            // 길이 1짜리(시작점만)도 완전 제곱수 판별
            let num = board[i][j] as i32;
            if is_perfect_square(num) {
                ans = ans.max(num);
            }
            for dr in -(n as isize - 1)..=(n as isize - 1) {
                for dc in -(m as isize - 1)..=(m as isize - 1) {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let mut r = i as isize;
                    let mut c = j as isize;
                    let mut num = 0;
                    while 0 <= r && r < n as isize && 0 <= c && c < m as isize {
                        num = num * 10 + board[r as usize][c as usize] as i32;
                        if is_perfect_square(num) {
                            ans = ans.max(num);
                        }
                        r += dr;
                        c += dc;
                    }
                }
            }
        }
    }
    writeln!(writer, "{}", ans).unwrap();
}

fn is_perfect_square(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let s = (x as f64).sqrt() as i32;
    s * s == x
}

// https://www.acmicpc.net/problem/1025
// 제곱수 찾기
#[test]
fn test_solve1025() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, tc) in [
        TestCase {
            s: "2 3
123
456"
            .to_string(),
            want: "64".to_string(),
        },
        TestCase {
            s: "5 5
00000
00000
00200
00000
00000"
                .to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "6 7
3791178
1283252
4103617
8233494
8725572
2937261"
                .to_string(),
            want: "320356".to_string(),
        },
        TestCase {
            s: "5 9
135791357
357913579
579135791
791357913
913579135"
                .to_string(),
            want: "9".to_string(),
        },
        TestCase {
            s: "9 9
553333733
775337775
777537775
777357333
755553557
355533335
373773573
337373777
775557777"
                .to_string(),
            want: "-1".to_string(),
        },
        TestCase {
            s: "9 9
257240281
197510846
014345401
035562575
974935632
865865933
684684987
768934659
287493867"
                .to_string(),
            want: "95481".to_string(),
        },
        // 추가 케이스
        TestCase {
            s: "4 4
2114
2924
2267
8252"
                .to_string(),
            want: "225".to_string(),
        },
        TestCase {
            s: "8 3
890
136
068
811
094
876
853
241"
            .to_string(),
            want: "361".to_string(),
        },
        TestCase {
            s: "5 7
2214551
6750680
6760405
2969225
8683696"
                .to_string(),
            want: "2401".to_string(),
        },
        TestCase {
            s: "1 3
162"
            .to_string(),
            want: "16".to_string(),
        },
        TestCase {
            s: "1 1
4"
            .to_string(),
            want: "4".to_string(),
        },
        TestCase {
            s: "1 1
1"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = tc.s.as_bytes();
        let mut writer = vec![];
        solve1025(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got.trim(), tc.want.trim(), "failed at {} with {}", i, tc.s);
    }
}
