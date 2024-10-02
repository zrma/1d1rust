use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11367(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let ans = (0..n)
        .map(|_| {
            let (name, score) = read_values_as!(read_line(reader), String, u32);
            let grade = match score {
                97..=100 => "A+",
                90..=96 => "A",
                87..=89 => "B+",
                80..=86 => "B",
                77..=79 => "C+",
                70..=76 => "C",
                67..=69 => "D+",
                60..=66 => "D",
                _ => "F",
            };
            format!("{} {}", name, grade)
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/11367
// noinspection SpellCheckingInspection
// Report Card Time
#[test]
fn test_solve11367() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
Bilbo 13
Sam 90
Pippin 78
Frodo 97
Merry 70"
                .to_string(),
            want: "Bilbo F
Sam A
Pippin C+
Frodo A+
Merry C"
                .to_string(),
        },
        TestData {
            s: "8
Gandalf 100
Gimli 59
Legolas 90
Boromir 80
Aragorn 90
Faramir 70
Denethor 60
Theoden 50"
                .to_string(),
            want: "Gandalf A+
Gimli F
Legolas A
Boromir B
Aragorn A
Faramir C
Denethor D
Theoden F"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11367(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
