use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
// noinspection SpellCheckingInspection
fn solve29766(reader: &mut impl BufRead, writer: &mut impl Write) {
    let input_line = read_line(reader);
    const SUBSTRING_TO_MATCH: &str = "DKSH";
    let match_count = input_line.matches(SUBSTRING_TO_MATCH).count();
    write!(writer, "{}", match_count).expect("Failed to write");
}

// https://www.acmicpc.net/problem/29766
// noinspection SpellCheckingInspection
// DKSH 찾기
#[test]
fn test_solve29766() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "DKKSSH".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "HDKSHDDKS".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "SDKSHSSDKSHS".to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve29766(&mut reader, &mut writer);

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
