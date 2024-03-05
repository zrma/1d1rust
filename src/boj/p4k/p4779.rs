use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4779(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap_or(0) > 0 {
        if let Ok(n) = line.trim().parse::<usize>() {
            let p = usize::pow(3, n as u32);
            let mut ans = vec!['-'; p];
            solve(n, &mut ans, 0, p - 1);

            writeln!(writer, "{}", ans.iter().collect::<String>()).unwrap();
        } else {
            break;
        }
        line.clear();
    }
}

fn solve(n: usize, ans: &mut Vec<char>, start: usize, end: usize) {
    if n == 0 {
        return;
    }
    let p = (end - start + 1) / 3;
    for it in &mut ans[start + p..start + p * 2] {
        *it = ' ';
    }
    solve(n - 1, ans, start, start + p - 1);
    solve(n - 1, ans, start + p * 2, end);
}

// https://www.acmicpc.net/problem/4779
// 칸토어 집합
#[test]
fn test_solve4779() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0
1
3
2"
            .to_string(),
            want: "-
- -
- -   - -         - -   - -
- -   - -
"
            .to_string(),
        },
        TestData {
            s: "4".to_string(),
            want:
                "- -   - -         - -   - -                           - -   - -         - -   - -
"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4779(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
