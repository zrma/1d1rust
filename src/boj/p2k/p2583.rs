use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2583(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (row, col, rects) = read_input(reader);

    let mut table = vec![vec![false; col as usize]; row as usize];
    fill_table(&mut table, &rects);

    let res = find_areas(&mut table);

    writeln!(writer, "{}", res.len()).unwrap();
    for n in res {
        write!(writer, "{} ", n).unwrap();
    }
}

fn find_areas(table: &mut [Vec<bool>]) -> Vec<i32> {
    let (row, col) = (table.len() as i32, table[0].len() as i32);
    let mut res: Vec<i32> = vec![];

    for y in 0..row {
        for x in 0..col {
            if !table[y as usize][x as usize] {
                let cnt = bfs(table, x, y);
                res.push(cnt);
            }
        }
    }

    res.sort_unstable();
    res
}

fn read_input(reader: &mut impl BufRead) -> (i32, i32, Vec<Vec<i32>>) {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut iter = line.split_whitespace();
    let row = iter.next().unwrap().parse::<i32>().unwrap();
    let col = iter.next().unwrap().parse::<i32>().unwrap();
    let n = iter.next().unwrap().parse::<i32>().unwrap();

    let rects = (0..n)
        .map(|_| {
            line.clear();
            reader.read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (row, col, rects)
}

fn fill_table(table: &mut [Vec<bool>], rects: &[Vec<i32>]) {
    for rect in rects {
        for y in rect[1]..rect[3] {
            for x in rect[0]..rect[2] {
                table[y as usize][x as usize] = true;
            }
        }
    }
}

fn bfs(table: &mut [Vec<bool>], x: i32, y: i32) -> i32 {
    struct Pos {
        x: i32,
        y: i32,
    }

    let mut cnt = 0;
    let mut queue = vec![Pos { x, y }];

    while let Some(pos) = queue.pop() {
        if table[pos.y as usize][pos.x as usize] {
            continue;
        }
        cnt += 1;
        table[pos.y as usize][pos.x as usize] = true;

        let moves = [
            (pos.x - 1, pos.y),
            (pos.x + 1, pos.y),
            (pos.x, pos.y - 1),
            (pos.x, pos.y + 1),
        ];

        for (next_x, next_y) in moves.iter() {
            if *next_x >= 0
                && *next_x < table[0].len() as i32
                && *next_y >= 0
                && *next_y < table.len() as i32
            {
                queue.push(Pos {
                    x: *next_x,
                    y: *next_y,
                });
            }
        }
    }

    cnt
}

// https://www.acmicpc.net/problem/2583
// 영역 구하기
#[test]
fn test_solve2583() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "5 7 3
0 2 4 4
1 1 2 5
4 0 6 2"
                .to_string(),
            want: "3
1 7 13 "
                .to_string(),
        },
        TestData {
            s: "100 100 0".to_string(),
            want: "1
10000 "
                .to_string(),
        },
        TestData {
            s: "100 100 1
0 0 1 1"
                .to_string(),
            want: "1
9999 "
                .to_string(),
        },
        TestData {
            s: "100 100 1
1 0 2 100"
                .to_string(),
            want: "2
100 9800 "
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2583(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
