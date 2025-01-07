use crate::read_values_as;
use crate::utils::io::read_line;
use num::pow::Pow;
use num::ToPrimitive;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4619(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let (b, n) = read_values_as!(read_line(reader), i32, i32);
        if b == 0 && n == 0 {
            break;
        }

        let power = n.to_f64().unwrap();
        let base = b.to_f64().unwrap();
        let estimated_a = base.powf(1.0 / power).round().to_i32().unwrap();

        let ans = (-1..=1)
            .map(|offset| estimated_a + offset)
            .filter(|&a| a >= 1)
            .min_by(|&a1, &a2| {
                let diff1 = (a1.to_f64().unwrap().pow(power) - base).abs();
                let diff2 = (a2.to_f64().unwrap().pow(power) - base).abs();
                diff1.partial_cmp(&diff2).unwrap()
            })
            .unwrap();

        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/4619
// 루트
#[test]
fn test_solve4619() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "4 3
5 3
27 3
750 5
1000 5
2000 5
3000 5
1000000 5
0 0"
            .to_string(),
            want: "1
2
3
4
4
4
5
16
"
            .to_string(),
        },
        TestData {
            s: "80 4
0 0"
            .to_string(),
            want: "3
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4619(&mut reader, &mut writer);

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
