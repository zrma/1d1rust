use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3568(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let mut iter = s.split_whitespace();

    let default_type = iter.next().unwrap();
    let mut vars = vec![];
    for mut var in iter {
        if var.ends_with(',') || var.ends_with(';') {
            var = &var[..var.len() - 1];
        }
        vars.push(var);
    }

    for var in vars {
        let var_name: String = var.chars().take_while(|&ch| ch.is_alphabetic()).collect();

        let others: String = var.chars().skip(var_name.len()).collect();
        if others.is_empty() {
            writeln!(writer, "{} {};", default_type, var_name).expect("Failed to write");
            continue;
        }

        let others: String = others.replace("[]", "][").chars().rev().collect();
        writeln!(writer, "{}{} {};", default_type, others, var_name).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/3568
// iSharp
#[test]
fn test_solve3568() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "int& a*[]&, b, c*;".to_string(),
        want: "int&&[]* a;
int& b;
int&* c;
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3568(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
