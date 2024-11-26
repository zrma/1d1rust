use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5354(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    for case in 0..num_cases {
        let n: usize = read_value(read_line(reader));

        if n == 1 {
            writeln!(writer, "#").expect("writeln! should work");
        } else {
            writeln!(writer, "{}", "#".repeat(n)).expect("writeln! should work");

            for _ in 1..n - 1 {
                writeln!(writer, "#{}#", "J".repeat(n - 2)).expect("writeln! should work");
            }
            writeln!(writer, "{}", "#".repeat(n)).expect("writeln! should work");
        }

        if case < num_cases - 1 {
            writeln!(writer).expect("writeln! should work");
        }
    }
}

// https://www.acmicpc.net/problem/5354
// J박스
#[test]
fn test_solve5354() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
3
5
4"
            .to_string(),
            want: "###
#J#
###

#####
#JJJ#
#JJJ#
#JJJ#
#####

####
#JJ#
#JJ#
####
"
            .to_string(),
        },
        TestData {
            s: "2
1
2"
            .to_string(),
            want: "#

##
##
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5354(&mut reader, &mut writer);

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
