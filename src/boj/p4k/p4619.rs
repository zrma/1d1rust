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

        let power = n.to_f64().expect("n should be convertible to f64");
        let base = b.to_f64().expect("b should be convertible to f64");
        let estimated_a = base
            .powf(1.0 / power)
            .round()
            .to_i32()
            .expect("a should be i32");

        let result = (-1..=1)
            .map(|offset| estimated_a + offset)
            .filter(|&a| a >= 1)
            .min_by(|&a1, &a2| {
                let diff1 = (a1
                    .to_f64()
                    .expect("a1 should be convertible to f64")
                    .pow(power)
                    - base)
                    .abs();
                let diff2 = (a2
                    .to_f64()
                    .expect("a2 should be convertible to f64")
                    .pow(power)
                    - base)
                    .abs();
                diff1
                    .partial_cmp(&diff2)
                    .expect("diff1 and diff2 should be comparable")
            })
            .expect("at least one value should be found");

        writeln!(writer, "{}", result).expect("writeln should work");
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

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
