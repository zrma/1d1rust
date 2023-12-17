use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15686(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values!(read_line(reader), usize, usize);

    let mut houses = Vec::new();
    let mut chickens = Vec::new();

    for y in 0..n {
        let row = read_line(reader);
        for (x, value) in row.split_whitespace().enumerate() {
            match value {
                "1" => houses.push((y, x)),
                "2" => chickens.push((y, x)),
                _ => {}
            }
        }
    }

    let mut min_city_distance = i32::MAX;
    let mut chicken_selection = vec![false; chickens.len()];
    choose_chickens(
        &houses,
        &chickens,
        &mut chicken_selection,
        0,
        m,
        &mut min_city_distance,
    );

    write!(writer, "{}", min_city_distance).unwrap();
}

fn choose_chickens(
    houses: &[(usize, usize)],
    chickens: &[(usize, usize)],
    selected: &mut Vec<bool>,
    start_index: usize,
    remaining: usize,
    min_distance: &mut i32,
) {
    if remaining == 0 {
        *min_distance = min_city_distance(houses, chickens, selected).min(*min_distance);
        return;
    }

    for i in start_index..chickens.len() {
        selected[i] = true;
        choose_chickens(
            houses,
            chickens,
            selected,
            i + 1,
            remaining - 1,
            min_distance,
        );
        selected[i] = false;
    }
}

fn min_city_distance(
    houses: &[(usize, usize)],
    chickens: &[(usize, usize)],
    selected: &[bool],
) -> i32 {
    houses
        .iter()
        .map(|&house| {
            chickens
                .iter()
                .enumerate()
                .filter(|&(i, _)| selected[i])
                .map(|(_, &chicken)| manhattan_distance(house, chicken))
                .min()
                .unwrap_or(i32::MAX)
        })
        .sum()
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> i32 {
    (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()
}

// https://www.acmicpc.net/problem/15686
// 치킨 배달
#[test]
fn test_solve15686() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 3
0 0 1 0 0
0 0 2 0 1
0 1 2 0 0
0 0 1 0 0
0 0 0 0 2"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "5 2
0 2 0 1 0
1 0 1 0 0
0 0 0 0 0
2 0 0 1 1
2 2 0 1 2"
                .to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "5 1
1 2 0 0 0
1 2 0 0 0
1 2 0 0 0
1 2 0 0 0
1 2 0 0 0"
                .to_string(),
            want: "11".to_string(),
        },
        TestData {
            s: "5 1
1 2 0 2 1
1 2 0 2 1
1 2 0 2 1
1 2 0 2 1
1 2 0 2 1"
                .to_string(),
            want: "32".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15686(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
