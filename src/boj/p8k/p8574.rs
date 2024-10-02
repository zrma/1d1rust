use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8574(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, radius, center_x, center_y) = read_values_as!(read_line(reader), usize, f32, f32, f32);

    let is_outside_circle = |(x, y): (f32, f32)| {
        let dx = center_x - x;
        let dy = center_y - y;
        dx * dx + dy * dy > radius * radius
    };

    let mut count_outside = 0;
    for _ in 0..n {
        let point = read_values_as!(read_line(reader), f32, f32);
        if is_outside_circle(point) {
            count_outside += 1;
        }
    }

    write!(writer, "{}", count_outside).unwrap();
}

// https://www.acmicpc.net/problem/8574
// Ratownik
#[test]
fn test_solve8574() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 3 2 2
2 4
2 6
3 3
4 2
5 6"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "4 3 2 2
2 4
2 6
3 3
4 2"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8574(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
