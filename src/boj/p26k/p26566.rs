use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve26566(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut s = String::new();
    let mut res = String::new();
    for _ in 0..n {
        s.clear();
        reader.read_line(&mut s).unwrap();

        let (a1, p1) = read_values_as!(&s, f64, f64);

        s.clear();
        reader.read_line(&mut s).unwrap();

        let (r1, p2) = read_values_as!(&s, f64, f64);

        let slice = a1 / p1;
        let whole = std::f64::consts::PI * r1 * r1 / p2;

        if slice > whole {
            res.push_str("Slice of pizza\n");
        } else {
            res.push_str("Whole pizza\n");
        }
    }

    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/26566
// Pizza
#[test]
fn test_solve26566() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
8 4
7 9
9 2
4 7
841 108
8 606"
                .to_string(),
            want: "Whole pizza
Whole pizza
Slice of pizza
"
            .to_string(),
        },
        TestData {
            s: "2
9 2
4 7
841 108
8 606"
                .to_string(),
            want: "Whole pizza
Slice of pizza
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve26566(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
