use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16401(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (required_amount, num_snacks) = read_values_as!(read_line(reader), u32, usize);
    let mut snack_lengths = read_n_values(reader, num_snacks);
    snack_lengths.sort_unstable();

    let optimal_length = find_optimal_length(&snack_lengths, required_amount);
    write!(writer, "{}", optimal_length).expect("Failed to write");
}

fn find_optimal_length(snack_lengths: &[u32], required_amount: u32) -> u32 {
    let (mut low, mut high) = (1, *snack_lengths.last().unwrap());
    let mut optimal_length = 0;

    while low <= high {
        let mid = (low + high) / 2;
        if can_satisfy(snack_lengths, required_amount, mid) {
            optimal_length = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    optimal_length
}

fn can_satisfy(snack_lengths: &[u32], required_amount: u32, length: u32) -> bool {
    snack_lengths
        .iter()
        .map(|&snack_length| snack_length / length)
        .sum::<u32>()
        >= required_amount
}

#[test]
fn test_solve16401() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 10
1 2 3 4 5 6 7 8 9 10"
                .to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "4 3
10 10 15"
                .to_string(),
            want: "7".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16401(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
