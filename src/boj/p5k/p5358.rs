use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5358(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut ans = Vec::new();
    let mut s = String::new();

    while reader.read_line(&mut s).unwrap_or(0) > 0 {
        if s.trim().is_empty() {
            break;
        }

        let transformed = s
            .trim_end()
            .chars()
            .map(|c| match c {
                'i' => 'e',
                'e' => 'i',
                'I' => 'E',
                'E' => 'I',
                _ => c,
            })
            .collect::<String>();
        ans.push(transformed);
        s.clear();
    }
    write!(writer, "{}", ans.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/5358
// noinspection SpellCheckingInspection
// Football Team
#[test]
fn test_solve5358() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "Alan Pagi
John Hiesman
Justen Forsitt
Phel Semms
Tem Tibow
Marshawn Lynch
Lion Washengton"
                .to_string(),
            want: "Alan Page
John Heisman
Justin Forsett
Phil Simms
Tim Tebow
Marshawn Lynch
Leon Washington"
                .to_string(),
        },
        TestData {
            s: "".to_string(),
            want: "".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5358(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
