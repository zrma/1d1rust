use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3107(reader: &mut impl BufRead, writer: &mut impl Write) {
    let ip = read_line(reader);
    let ans = expand_ipv6(&ip);
    write!(writer, "{}", ans).unwrap();
}

fn expand_ipv6(ip: &str) -> String {
    let expanded = if ip.contains("::") {
        expand_double_colon(ip)
    } else {
        ip.to_string()
    };

    fill_zero(&expanded)
}

fn expand_double_colon(ip: &str) -> String {
    let colon_count = ip.matches(':').count() - 2;
    let need_group_count = 6 - colon_count;
    let colon_expanded =
        ":".to_string() + "0000:".repeat(need_group_count).trim_end_matches(':') + ":";

    ip.replace("::", &colon_expanded).replace("::", ":")
}

fn fill_zero(ip: &str) -> String {
    ip.split(':')
        .map(|part| format!("{:0>4}", part))
        .collect::<Vec<_>>()
        .join(":")
}

// https://www.acmicpc.net/problem/3107
// IPv6
#[test]
fn test_solve3107() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "25:09:1985:aa:091:4846:374:bb".to_string(),
            want: "0025:0009:1985:00aa:0091:4846:0374:00bb".to_string(),
        },
        TestData {
            s: "::1".to_string(),
            want: "0000:0000:0000:0000:0000:0000:0000:0001".to_string(),
        },
        TestData {
            s: "1::".to_string(),
            want: "0001:0000:0000:0000:0000:0000:0000:0000".to_string(),
        },
        TestData {
            s: "0:0:0:0:0:0:0:0".to_string(),
            want: "0000:0000:0000:0000:0000:0000:0000:0000".to_string(),
        },
        TestData {
            s: "1:2:3:4:5:6:7::".to_string(),
            want: "0001:0002:0003:0004:0005:0006:0007:0000".to_string(),
        },
        TestData {
            s: "::".to_string(),
            want: "0000:0000:0000:0000:0000:0000:0000:0000".to_string(),
        },
        TestData {
            s: "1::1".to_string(),
            want: "0001:0000:0000:0000:0000:0000:0000:0001".to_string(),
        },
        TestData {
            s: "1::1:1".to_string(),
            want: "0001:0000:0000:0000:0000:0000:0001:0001".to_string(),
        },
        TestData {
            s: "1:1::1".to_string(),
            want: "0001:0001:0000:0000:0000:0000:0000:0001".to_string(),
        },
        TestData {
            s: "1:2::2:1".to_string(),
            want: "0001:0002:0000:0000:0000:0000:0002:0001".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3107(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
