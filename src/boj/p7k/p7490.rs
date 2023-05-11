use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve7490(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();

    for _ in 0..t {
        let n = read_line(reader).parse::<usize>().unwrap();

        make_to_zero(writer, n);
        writeln!(writer).unwrap();
    }
}

fn make_to_zero(writer: &mut impl Write, n: usize) {
    let mut nums = (1..=n).map(|i| i.to_string()).collect();

    let mut res = vec![];
    make_to_zero_rec(&mut res, &mut nums, 0);

    for s in res {
        writeln!(writer, "{}", s).unwrap();
    }
}

fn make_to_zero_rec(res: &mut Vec<String>, nums: &mut Vec<String>, depth: usize) {
    if depth == nums.len() - 1 {
        let s = nums.join("");
        let n = eval(&s);
        if n == 0 {
            res.push(s);
        }
        return;
    }

    for &op in &[' ', '+', '-'] {
        nums[depth].push(op);
        make_to_zero_rec(res, nums, depth + 1);
        nums[depth].pop();
    }
}

fn eval(expr: &str) -> i32 {
    let expr = expr.replace(' ', "");

    let mut nums = Vec::new();
    let mut ops = Vec::new();
    let mut num = 0;

    for c in expr.chars() {
        if c.is_ascii_digit() {
            num = num * 10 + c.to_digit(10).unwrap() as i32;
        } else {
            nums.push(num);
            num = 0;
            ops.push(c);
        }
    }
    nums.push(num);

    let mut result = nums[0];
    for i in 0..ops.len() {
        if ops[i] == '+' {
            result += nums[i + 1];
        } else {
            result -= nums[i + 1];
        }
    }
    result
}

// https://www.acmicpc.net/problem/7490
// 0 만들기
#[test]
fn test_solve7490() {
    struct TestData {
        s: String,
        want: String,
    }
    for data in vec![TestData {
        s: "2
3
7"
        .to_string(),
        want: "1+2-3

1+2-3+4-5-6+7
1+2-3-4+5+6-7
1-2 3+4+5+6+7
1-2 3-4 5+6 7
1-2+3+4-5+6-7
1-2-3-4-5+6+7

"
        .to_string(),
    }] {
        use std::io::Cursor;
        let mut reader = Cursor::new(data.s);
        let mut writer = Cursor::new(Vec::new());
        solve7490(&mut reader, &mut writer);

        let got = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(data.want, got);
    }
}