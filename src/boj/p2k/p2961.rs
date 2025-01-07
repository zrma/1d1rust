use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2961(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut ingredients: Vec<(u32, u32)> = vec![];

    for _ in 0..n {
        let (sour, bitter) = read_values_as!(read_line(reader), u32, u32);
        ingredients.push((sour, bitter));
    }

    let mut min_diff = u32::MAX;
    for mask in 1..(1 << n) {
        let mut total_sour = 1;
        let mut total_bitter = 0;
        for (idx, (sour, bitter)) in ingredients.iter().enumerate() {
            if mask & (1 << idx) != 0 {
                total_sour *= sour;
                total_bitter += bitter;
            }
        }
        min_diff = min_diff.min(total_sour.abs_diff(total_bitter));
    }

    writeln!(writer, "{}", min_diff).unwrap();
}

// https://www.acmicpc.net/problem/2961
// 도영이가 만든 맛있는 음식
#[test]
fn test_solve2961() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
3 10"
                .to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "2
3 8
5 8"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "4
1 7
2 6
3 8
4 9"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = vec![];
            solve2961(&mut reader, &mut writer);

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
}
