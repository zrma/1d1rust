use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2669(reader: &mut impl BufRead, writer: &mut impl Write) {
    let rects: Vec<Rect> = (0..4)
        .map(|_| read_values_as!(read_line(reader), i32, i32, i32, i32))
        .collect::<Vec<_>>();

    // 모든 직사각형의 x좌표 수집 후 정렬 & 중복 제거
    let mut xs = Vec::new();
    for &(x1, _, x2, _) in &rects {
        xs.push(x1);
        xs.push(x2);
    }
    xs.sort_unstable();
    xs.dedup();

    let mut total_area = 0;

    // 인접한 x좌표 쌍 [xs[i], xs[i+1]]에 대해 처리
    for i in 0..xs.len() - 1 {
        let x_left = xs[i];
        let x_right = xs[i + 1];

        let width = x_right - x_left;
        if width <= 0 {
            continue;
        }

        // 현재 세로 띠와 겹치는 사각형들의 y구간들을 모아서 병합
        let mut intervals: Vec<(i32, i32)> = Vec::new();

        for &(rx1, ry1, rx2, ry2) in &rects {
            // 이 사각형이 x축으로 [x_left, x_right] 범위와 교집합을 가지는지 확인
            if rx2 <= x_left || rx1 >= x_right {
                // 교집합 없는 경우
                continue;
            }
            // 교집합이 있으면 해당 y구간 추가
            intervals.push((ry1, ry2));
        }

        //  y구간들 정렬 후 서로 겹치는 부분 병합
        intervals.sort_unstable_by_key(|(start, end)| (*start, *end));

        let mut merged_length = 0;
        let mut current_start_end: Option<(i32, i32)> = None;

        for (start, end) in intervals {
            match current_start_end {
                None => {
                    // 첫 구간이므로 채택
                    current_start_end = Some((start, end));
                }
                Some((cs, ce)) => {
                    if start > ce {
                        // 이전 구간과 전혀 겹치지 않으면, 앞 구간 길이를 확정하고 새 구간 시작
                        merged_length += ce - cs;
                        current_start_end = Some((start, end));
                    } else {
                        // 겹칠 경우, end가 더 크면 확장
                        let new_end = ce.max(end);
                        current_start_end = Some((cs, new_end));
                    }
                }
            }
        }

        // 마지막 구간 처리
        if let Some((cs, ce)) = current_start_end {
            merged_length += ce - cs;
        }

        // 세로 띠 면적 = (병합된 y 길이) × (x 폭)
        total_area += merged_length * width;
    }

    // 결과 출력
    writeln!(writer, "{}", total_area).unwrap();
}

type Rect = (i32, i32, i32, i32);

// https://www.acmicpc.net/problem/2669
// 직사각형 네개의 합집합의 면적 구하기
#[test]
fn test_solve2669() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "1 2 4 4
2 3 5 7
3 1 6 5
7 3 8 6"
                .to_string(),
            want: "26".to_string(),
        },
        TestCase {
            s: "1 1 2 2
2 2 3 3
3 3 4 4
4 4 5 5"
                .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2669(&mut reader, &mut writer);

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
