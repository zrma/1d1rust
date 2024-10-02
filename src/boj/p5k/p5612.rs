use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5612(reader: &mut impl BufRead, writer: &mut impl Write) {
    let tick: usize = read_value(read_line(reader));
    let init_cars: i32 = read_value(read_line(reader));

    let mut max_cars = init_cars;
    let mut curr_cars = init_cars;
    for _ in 0..tick {
        let (in_, out) = read_values_as!(read_line(reader), i32, i32);
        curr_cars += in_ - out;
        if curr_cars < 0 {
            write!(writer, "0").expect("Failed to write");
            return;
        }
        max_cars = max_cars.max(curr_cars);
    }

    write!(writer, "{}", max_cars).expect("Failed to write");
}

// https://www.acmicpc.net/problem/5612
// 터널의 입구와 출구
#[test]
fn test_solve5612() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
2
2 3
2 3
4 1"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "3
2
2 3
2 4
4 1"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "3
2
2 3
2 3
1 0"
            .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5612(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
