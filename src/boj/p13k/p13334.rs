use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13334(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut lines = Vec::with_capacity(n);

    for _ in 0..n {
        let (h, o): (i32, i32) = read_values_as!(read_line(reader), i32, i32);
        lines.push((h.min(o), h.max(o)));
    }

    let d: i32 = read_value(read_line(reader));

    // 선분의 끝점을 기준으로 정렬
    lines.sort_unstable_by_key(|&(_, end)| end);

    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut max_count = 0;

    for &(start, end) in lines.iter() {
        // 현재 선분의 길이가 d보다 크면 무시
        if end - start > d {
            continue;
        }

        // 현재 선분의 끝점을 기준으로 철로를 배치했을 때,
        // 철로의 시작점(end - d)보다 앞에 있는 선분들을 제거
        while let Some(&Reverse(top)) = heap.peek() {
            if top < end - d {
                heap.pop();
            } else {
                break;
            }
        }

        // 현재 선분의 시작점을 힙에 추가
        heap.push(Reverse(start));
        max_count = max_count.max(heap.len());
    }

    writeln!(writer, "{}", max_count).unwrap();
}

// https://www.acmicpc.net/problem/13334
// 철로
#[test]
fn test_solve13334() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "8
5 40
35 25
10 20
10 25
30 50
50 60
30 25
80 100
30"
            .to_string(),
            want: "4".to_string(),
        },
        TestCase {
            s: "4
20 80
70 30
35 65
40 60
10"
            .to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "5
-5 5
30 40
-5 5
50 40
5 -5
10"
            .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13334(&mut reader, &mut writer);

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
