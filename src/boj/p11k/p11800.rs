use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11800(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    (1..=num_cases).for_each(|i| {
        let (a, b): (u8, u8) = read_values_as!(read_line(reader), u8, u8);
        let (a, b) = (a.max(b), a.min(b));

        if a == b {
            writeln!(writer, "Case {}: {}", i, get_double_name(a)).expect("write! should work");
            return;
        }

        if a == 6 && b == 5 {
            writeln!(writer, "Case {}: Sheesh Beesh", i).expect("write! should work");
            return;
        }

        writeln!(
            writer,
            "Case {}: {} {}",
            i,
            get_single_name(a),
            get_single_name(b)
        )
        .expect("write! should work");
    });
}

fn get_double_name(value: u8) -> &'static str {
    match value {
        1 => "Habb Yakk",
        2 => "Dobara",
        3 => "Dousa",
        4 => "Dorgy",
        5 => "Dabash",
        6 => "Dosh",
        _ => unreachable!("invalid value"),
    }
}

fn get_single_name(value: u8) -> &'static str {
    match value {
        1 => "Yakk",
        2 => "Doh",
        3 => "Seh",
        4 => "Ghar",
        5 => "Bang",
        6 => "Sheesh",
        _ => unreachable!("invalid value"),
    }
}

// https://www.acmicpc.net/problem/11800
// noinspection SpellCheckingInspection
// Tawla
#[test]
fn test_solve11800() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
1 2
2 3
3 4"
            .to_string(),
            want: "Case 1: Doh Yakk
Case 2: Seh Doh
Case 3: Ghar Seh
"
            .to_string(),
        },
        TestData {
            s: "9
1 1
2 2
3 3
4 4
5 5
6 6
4 5
5 6
1 6"
            .to_string(),
            want: "Case 1: Habb Yakk
Case 2: Dobara
Case 3: Dousa
Case 4: Dorgy
Case 5: Dabash
Case 6: Dosh
Case 7: Bang Ghar
Case 8: Sheesh Beesh
Case 9: Sheesh Yakk
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11800(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
