use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3447(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut answers = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap_or(0) > 0 {
        let mut s = line.trim().to_string();
        while s.contains("BUG") {
            s = s.replace("BUG", "");
        }
        answers.push(s);
        line.clear();
    }

    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/3447
// noinspection SpellCheckingInspection
// 버그왕
#[test]
fn test_solve3447() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "print \"No bugs here...\"

void hello() {
BUGBUG
printfBUG(\"Hello, world!\\n\");
}

wriBUGBUGtelBUGn(\"Hello B-U-G\");"
                .to_string(),
            want: "print \"No bugs here...\"

void hello() {

printf(\"Hello, world!\\n\");
}

writeln(\"Hello B-U-G\");"
                .to_string(),
        },
        TestData {
            s: "ABUBUGGB".to_string(),
            want: "AB".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve3447(&mut reader, &mut writer);

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
