use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3554(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut arr: Vec<usize> = read_n_values(reader, n);

    let m: usize = read_value(read_line(reader));

    for _ in 0..m {
        let (k, l, r) = read_values_as!(read_line(reader), u8, usize, usize);

        if let Some(slice) = arr.get_mut(l - 1..r) {
            match k {
                1 => slice
                    .iter_mut()
                    .for_each(|v| *v = v.wrapping_mul(*v) % 2010),
                _ => {
                    let sum: usize = slice.iter().sum();
                    writeln!(writer, "{}", sum).expect("Failed to write");
                }
            }
        }
    }
}

// https://www.acmicpc.net/problem/3554
// Enigmatic Device
#[test]
fn test_solve3554() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
17 239 999
4
2 1 3
1 2 3
2 2 3
2 1 2"
                .to_string(),
            want: "1255
1882
858
"
            .to_string(),
        },
        TestData {
            s: "3
17 239 999
1
2 1 3"
                .to_string(),
            want: "1255
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = vec![];
            solve3554(&mut reader, &mut writer);

            let got = String::from_utf8(writer).expect("writer should be a valid string");
            assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
        }
    }
}
