use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10173(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let s = read_line(reader);
        if s == "EOI" {
            break;
        }

        let ans = {
            if s.to_lowercase().contains("nemo") {
                "Found"
            } else {
                "Missing"
            }
        };

        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/10173
// 니모를 찾아서
#[test]
fn test_solve10173() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![TestData {
        s: "Marlin names this last egg Nemo, a name that Coral liked.
While attempting to save nemo, Marlin meets Dory,
a good-hearted and optimistic regal blue tang with short-term memory loss.
Upon leaving the East Australian Current,(888*%$^&%0928375)Marlin and Dory
NEMO leaves for school and Marlin watches NeMo swim away.
EOI"
        .to_string(),
        want: "Found
Found
Missing
Missing
Found
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10173(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
