use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15592(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut target = read_rect(reader);
    let blocker = read_rect(reader);

    if blocker.covers_horizontally(&target) {
        if blocker.contains_vertical(target.bottom) {
            target.bottom = blocker.top;
        }
        if blocker.contains_vertical(target.top) {
            target.top = blocker.bottom;
        }
    }

    if blocker.covers_vertically(&target) {
        if blocker.contains_horizontal(target.left) {
            target.left = blocker.right;
        }
        if blocker.contains_horizontal(target.right) {
            target.right = blocker.left;
        }
    }

    let res = target.area();
    writeln!(writer, "{}", res).unwrap();
}

struct Rect {
    left: i32,
    bottom: i32,
    right: i32,
    top: i32,
}

impl Rect {
    fn covers_horizontally(&self, other: &Rect) -> bool {
        self.contains_horizontal(other.left) && self.contains_horizontal(other.right)
    }

    fn covers_vertically(&self, other: &Rect) -> bool {
        self.contains_vertical(other.bottom) && self.contains_vertical(other.top)
    }

    fn contains_horizontal(&self, val: i32) -> bool {
        is_within(val, self.left, self.right)
    }

    fn contains_vertical(&self, val: i32) -> bool {
        is_within(val, self.bottom, self.top)
    }

    fn area(&self) -> i32 {
        (self.right - self.left).max(0) * (self.top - self.bottom).max(0)
    }
}

fn is_within(val: i32, min: i32, max: i32) -> bool {
    min <= val && val <= max
}

fn read_rect(reader: &mut impl BufRead) -> Rect {
    let (left, bottom, right, top) = read_values_as!(read_line(reader), i32, i32, i32, i32);
    Rect {
        left,
        bottom,
        right,
        top,
    }
}

// https://www.acmicpc.net/problem/15592
// Blocked Billboard II
#[test]
fn test_solve15592() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 1 7 4
5 -1 10 3"
                .to_string(),
            want: "15".to_string(),
        },
        TestData {
            s: "0 0 10 10
0 -0 9 10"
                .to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "0 0 10 10
0 0 10 10"
                .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15592(&mut reader, &mut writer);

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
