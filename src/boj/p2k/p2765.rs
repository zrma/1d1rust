use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2765(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut trip = 0;
    let mut answers = Vec::new();

    loop {
        trip += 1;
        let (diameter_inch, revolutions, time_sec) =
            read_values_as!(read_line(reader), f64, f64, f64);
        if revolutions == 0.0 {
            break;
        }

        let distance = calculate_distance(diameter_inch, revolutions);
        let speed = calculate_speed(distance, time_sec);

        answers.push(format!("Trip #{}: {:.2} {:.2}", trip, distance, speed));
    }

    write!(writer, "{}", answers.join("\n")).unwrap();
}

fn calculate_distance(diameter_inch: f64, revolutions: f64) -> f64 {
    diameter_inch * std::f64::consts::PI * revolutions / 63360.0
}

fn calculate_speed(distance: f64, time_sec: f64) -> f64 {
    distance / (time_sec / 3600.0)
}

// https://www.acmicpc.net/problem/2765
// 자전거 속도
#[test]
fn test_solve2765() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "26 1000 5
27.25 873234 3000
26 0 1000"
                .to_string(),
            want: "Trip #1: 1.29 928.20
Trip #2: 1179.86 1415.84"
                .to_string(),
        },
        TestData {
            s: "2016.812 10 3600
2016.812 0 1000"
                .to_string(),
            want: "Trip #1: 1.00 1.00".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2765(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
