use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6236(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);
    let expenses: Vec<usize> = (0..n).map(|_| read_line(reader).parse().unwrap()).collect();
    let max_expense = *expenses.iter().max().unwrap();
    let total_expenses: usize = expenses.iter().sum();

    let optimal_withdrawal =
        find_optimal_withdrawal_amount(&expenses, max_expense, total_expenses, m);
    write!(writer, "{}", optimal_withdrawal).unwrap();
}

fn find_optimal_withdrawal_amount(
    expenses: &[usize],
    mut left: usize,
    mut right: usize,
    m: usize,
) -> usize {
    while left < right {
        let mid = (left + right) / 2;
        let mut withdrawal_count = 1;
        let mut current_sum = 0;

        for &expense in expenses {
            if current_sum + expense > mid {
                withdrawal_count += 1;
                current_sum = 0;
            }
            current_sum += expense;
        }

        if withdrawal_count <= m {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

// https://www.acmicpc.net/problem/6236
// 용돈 관리
#[test]
fn test_solve6236() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7 5
100
400
300
100
500
101
400"
            .to_string(),
            want: "500".to_string(),
        },
        TestData {
            s: "4 4
100
100
100
100"
            .to_string(),
            want: "100".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6236(&mut reader, &mut writer);

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
