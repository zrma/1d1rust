use crate::read_values;
use crate::utils::io::{read_line, read_map};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14503(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (rows, cols) = read_values!(read_line(reader), usize, usize);
    let (start_row, start_col, start_dir) = read_values!(read_line(reader), usize, usize, usize);

    let mut map = read_map(reader, rows, cols);

    let mut cleaner = Cleaner::new(start_col, start_row, start_dir, &mut map);
    cleaner.clean();

    write!(writer, "{}", cleaner.count).unwrap();
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_index(index: usize) -> Self {
        use Direction::*;
        match index {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => unreachable!(),
        }
    }

    fn left(self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn back(self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    fn delta(self) -> (i32, i32) {
        use Direction::*;
        match self {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        }
    }
}

struct Cleaner<'a> {
    x: usize,
    y: usize,
    direction: Direction,
    map: &'a mut Vec<Vec<usize>>,
    count: usize,
}

impl<'a> Cleaner<'a> {
    fn new(x: usize, y: usize, direction_index: usize, map: &'a mut Vec<Vec<usize>>) -> Self {
        let direction = Direction::from_index(direction_index);

        Self {
            x,
            y,
            direction,
            map,
            count: 0,
        }
    }

    fn clean(&mut self) {
        loop {
            self.clean_current();

            if !self.has_cleanable_around() {
                if !self.try_to_rewind() {
                    break;
                }
                continue;
            }

            self.turn_left();
            self.go_forward();
        }
    }

    fn clean_current(&mut self) {
        if self.map[self.y][self.x] == 0 {
            self.map[self.y][self.x] = 2;
            self.count += 1;
        }
    }

    fn has_cleanable_around(&self) -> bool {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .any(|&d| {
            let (dx, dy) = d.delta();
            let nx = self.x as i32 + dx;
            let ny = self.y as i32 + dy;
            self.is_cleanable(nx, ny)
        })
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.left();
    }

    fn go_forward(&mut self) {
        let (dx, dy) = self.direction.delta();
        let nx = self.x as i32 + dx;
        let ny = self.y as i32 + dy;

        if self.is_cleanable(nx, ny) {
            self.x = nx as usize;
            self.y = ny as usize;
        }
    }

    fn try_to_rewind(&mut self) -> bool {
        let (dx, dy) = self.direction.back().delta();
        let nx = self.x as i32 + dx;
        let ny = self.y as i32 + dy;

        if self.is_rewindable(nx, ny) {
            self.x = nx as usize;
            self.y = ny as usize;
            true
        } else {
            false
        }
    }

    fn is_in_bounds(&self, nx: i32, ny: i32) -> bool {
        ny >= 0 && ny < self.map.len() as i32 && nx >= 0 && nx < self.map[0].len() as i32
    }

    fn is_cleanable(&self, x: i32, y: i32) -> bool {
        self.is_in_bounds(x, y) && self.map[y as usize][x as usize] == 0
    }

    fn is_rewindable(&self, x: i32, y: i32) -> bool {
        self.is_in_bounds(x, y) && self.map[y as usize][x as usize] != 1
    }
}

// https://www.acmicpc.net/problem/14503
// 로봇 청소기
#[test]
fn test_solve14503() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 3
1 1 0
1 1 1
1 0 1
1 1 1"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "11 10
7 4 0
1 1 1 1 1 1 1 1 1 1
1 0 0 0 0 0 0 0 0 1
1 0 0 0 1 1 1 1 0 1
1 0 0 1 1 0 0 0 0 1
1 0 1 1 0 0 0 0 0 1
1 0 0 0 0 0 0 0 0 1
1 0 0 0 0 0 0 1 0 1
1 0 0 0 0 0 1 1 0 1
1 0 0 0 0 0 1 1 0 1
1 0 0 0 0 0 0 0 0 1
1 1 1 1 1 1 1 1 1 1"
                .to_string(),
            want: "57".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14503(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
