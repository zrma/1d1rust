use crate::utils::functions::char_to_index;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2257(reader: &mut impl BufRead, writer: &mut impl Write) {
    let formula = read_line(reader);

    let total_mass = calculate_mass(&formula);

    write!(writer, "{}", total_mass).unwrap();
}

fn calculate_mass(formula: &str) -> i32 {
    let mut stack = Vec::new();
    for element in formula.chars() {
        match element {
            '(' => stack.push(0),
            ')' => {
                let mut compound_mass = 0;
                while let Some(mass) = stack.pop() {
                    if mass == 0 {
                        break;
                    }
                    compound_mass += mass;
                }
                stack.push(compound_mass);
            }
            'H' => stack.push(1),
            'C' => stack.push(12),
            'O' => stack.push(16),
            digit => {
                let mass: u32 = char_to_index(digit);
                if let Some(last) = stack.pop() {
                    stack.push(last * mass as i32);
                }
            }
        }
    }

    stack.iter().sum()
}

// https://www.acmicpc.net/problem/2257
// noinspection SpellCheckingInspection
// 화학식량
#[test]
fn test_solve2257() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "(H)2(O)".to_string(),
            want: "18".to_string(),
        },
        TestData {
            s: "CH(CO2H)3".to_string(),
            want: "148".to_string(),
        },
        TestData {
            s: "((CH)2(OH2H)(C(H))O)3".to_string(),
            want: "222".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2257(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
