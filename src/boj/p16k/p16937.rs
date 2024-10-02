use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16937(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (h, w) = read_values_as!(read_line(reader), i32, i32);

    let n = read_value(read_line(reader));

    let mut stickers = Vec::new();
    for _ in 0..n {
        stickers.push(read_values_as!(read_line(reader), i32, i32));
    }

    let res = max_area(h, w, stickers);
    write!(writer, "{}", res).expect("Failed to write");
}

fn max_area(h: i32, w: i32, stickers: Vec<(i32, i32)>) -> i32 {
    let mut max_area = 0;
    for i in 0..stickers.len() {
        for j in i + 1..stickers.len() {
            let (a, b) = stickers[i];
            let (c, d) = stickers[j];

            let area = calc_area(h, w, a, b, c, d);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn calc_area(h: i32, w: i32, a: i32, b: i32, c: i32, d: i32) -> i32 {
    if a + c <= h && b.max(d) <= w
        || a.max(c) <= h && b + d <= w
        || a + d <= h && b.max(c) <= w
        || a.max(d) <= h && b + c <= w
        || b + c <= h && a.max(d) <= w
        || b.max(c) <= h && a + d <= w
        || b + d <= h && a.max(c) <= w
        || b.max(d) <= h && a + c <= w
    {
        a * b + c * d
    } else {
        0
    }
}

// https://www.acmicpc.net/problem/16937
// 두 스티커
#[test]
fn test_solve16937() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 2
2
1 2
2 1"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "10 9
4
2 3
1 1
5 10
9 11"
                .to_string(),
            want: "56".to_string(),
        },
        TestData {
            s: "10 10
3
6 6
7 7
20 5"
                .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16937(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
