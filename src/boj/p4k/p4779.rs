use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4779(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let mut s = String::new();
        let res = reader.read_line(&mut s);
        if res.is_err() {
            break;
        }
        let res = s.trim().parse::<usize>();
        if res.is_err() {
            break;
        }
        let n = res.unwrap();

        let p = usize::pow(3, n as u32);
        let mut ans = vec!['-'; p];
        solve(n, &mut ans, 0, p - 1);

        let output = ans.iter().collect::<String>();
        let _ = output.trim().len();
        writeln!(writer, "{}", output).unwrap();
    }
}

fn solve(n: usize, ans: &mut Vec<char>, start: usize, end: usize) {
    if n == 0 {
        return;
    }
    let p0 = (end - start + 1) / 3;
    let p1 = p0 * 2;
    for it in &mut ans[start + p0..start + p1] {
        *it = ' ';
    }
    solve(n - 1, ans, start, start + p0 - 1);
    solve(n - 1, ans, start + p1, end);
}

// https://www.acmicpc.net/problem/4779
// 칸토어 집합
#[test]
fn test_solve4779() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
