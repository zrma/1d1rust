use crate::utils::io::{read_line, read_n_values, read_value};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2075(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut min_heap = BinaryHeap::with_capacity(n);

    for _ in 0..n {
        let nums: Vec<i32> = read_n_values(reader, n);

        for x in nums {
            if min_heap.len() < n {
                min_heap.push(Reverse(x));
            } else if let Some(&Reverse(top)) = min_heap.peek() {
                if x > top {
                    min_heap.pop();
                    min_heap.push(Reverse(x));
                }
            }
        }
    }

    if let Some(&Reverse(ans)) = min_heap.peek() {
        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/2075
// N번째 큰 수
#[test]
fn test_solve2075() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
12 7 9 15 5
13 8 11 19 6
21 10 26 31 16
48 14 28 35 25
52 20 32 41 49"
                .to_string(),
            want: "35".to_string(),
        },
        TestData {
            s: "1
1"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1
0"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "2
1 2
3 4"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "2
3 1
4 2"
            .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2075(&mut reader, &mut writer);

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
