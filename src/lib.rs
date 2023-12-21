#![allow(dead_code)]
use std::fmt;
use colored::*;
use Stiker::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Stiker {
    B,  // Blue
    R,  // Red
    Y,  // Yellow
    O,  // Orange
    G,  // Green
    W,  // White
    V   // Void
}

impl Into<Stiker> for char {
    fn into(self) -> Stiker {
        match self {
            'b' => B,
            'r' => R,
            'y' => Y,
            'o' => O,
            'g' => G,
            'w' => W,
            _ => V
        }
    }
}

impl Into<char> for Stiker {
    fn into(self) -> char {
        match self {
            B => 'b',
            R => 'r',
            Y => 'y',
            O => 'o',
            G => 'g',
            W => 'w',
            V => ' '
        }
    }
}

type Scan = [[Stiker; 12]; 9];

#[derive(PartialEq)]
pub struct Cube {
    pub scan: Scan
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "\n  0 2 4 6 8 10".to_string();
        let string = format!("{}", self);
        let result_chars: Vec<_> = string.lines().collect();
        for i in 1..result_chars.len() {
            result = format!("{}\n{} {}", result, i - 1, result_chars[i]);
        }
        write!(f, "{}", result)
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "\n".to_string();
        for layer in self.scan {
            for letter in layer {
                let letter_char: char = letter.into();
                result = format!("{}{}", result, match letter_char {
                    'b' => "b".blue(),
                    'r' => "r".red(),
                    'y' => "y".truecolor(255, 255, 0),
                    'o' => "o".truecolor(255, 135, 0),
                    'g' => "g".green(),
                    'w' => "w".white(),
                    _ => " ".normal()
                })
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::from_scan([
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
        ])
    }
}

impl Cube {
    fn from(string: impl Into<String>) -> Self {
        let mut cube = Cube::default();
        let borrow = string.into();
        let notes = borrow.split_whitespace();
        for note in notes {
            match note {
                "R"             => cube.rotate_rigth(),
                "R'"            => cube.rotate_rigth_inv(),
                "R2" | "R2'"    => for _ in 0..2 { cube.rotate_rigth() },
                "L"             => cube.rotate_left(),
                "L'"            => cube.rotate_left_inv(),
                "L2" | "L2'"    => for _ in 0..2 { cube.rotate_left() },
                "U"             => cube.rotate_up(),
                "U'"            => cube.rotate_up_inv(),
                "U2" | "U2'"    => for _ in 0..2 { cube.rotate_up() },
                "F"             => cube.rotate_front(),
                "F'"            => cube.rotate_front_inv(),
                "F2" | "F2'"    => for _ in 0..2 { cube.rotate_front() },
                "D"             => cube.rotate_down(),
                "D'"            => cube.rotate_down_inv(),
                "D2" | "D2'"    => for _ in 0..2 { cube.rotate_down() },
                "B"             => cube.rotate_back(),
                "B'"            => cube.rotate_back_inv(),
                "B2" | "B2'"    => for _ in 0..2 { cube.rotate_back() },
                "M"             => cube.rotate_middle(),
                "M'"            => cube.rotate_middle_inv(),
                "M2" | "M2'"    => for _ in 0..2 { cube.rotate_middle() }
                _ => ()
            }
        }
        cube
    }
}

impl Cube {
    fn from_scan(inner_scan: [[char; 12]; 9]) -> Self {
        Cube {
            scan: inner_scan.map(|x| x.map(|x| x.into()))
        }
    }
}

fn instr_by_center(center: (usize, usize)) -> [[(usize, usize); 4]; 2] {
    let x = center.0;
    let y = center.1;
    [[(x - 1, y + 1), (x - 1, y - 1), (x + 1, y - 1), (x + 1, y + 1)], 
    [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]]
}

impl Cube {
    fn rotate_by_instr(&mut self, instr: [(usize, usize); 4]) {
        let first = instr.first().unwrap();
        let buffer = self.scan[first.0][first.1];
        for i in 0..=2 {
            let now = instr[i];
            let next = instr[i + 1];
            self.scan[now.0][now.1] = self.scan[next.0][next.1];
        }
        let last = instr.last().unwrap();
        self.scan[last.0][last.1] = buffer;
    }
    fn rotate_center(&mut self, center: (usize, usize)) {
        for instr in instr_by_center(center) {
            self.rotate_by_instr(instr);
        }
    }
    fn rotate_center_inv(&mut self, center: (usize, usize)) {
        for mut instr in instr_by_center(center) {
            instr.reverse();
            self.rotate_by_instr(instr);
        }
    }
    fn rotate_by_start_and_direction(
        &mut self, 
        start: [(isize, isize); 4],
        start_direction: [(isize, isize); 4]
    ) {
        for i in 0..=2 {
            let mut instr = [(0, 0); 4];
            for j in 0..=3 {
                let rotate = start[j];
                let direction = start_direction[j];
                instr[j] = (
                    (rotate.0 + direction.0 * i as isize) as usize, 
                    (rotate.1 + direction.1 * i as isize) as usize);
            }
            self.rotate_by_instr(instr);
        }
    }
    fn rotate_by_start_and_direction_inv(
        &mut self,
        start: [(isize, isize); 4],
        start_direction: [(isize, isize); 4]
    ) {
        let mut start_inv = start;
        start_inv.reverse();
        let mut direction_inv = start_direction;
        direction_inv.reverse();
        self.rotate_by_start_and_direction(start_inv, direction_inv);
    }
    pub fn rotate_front(&mut self) {
        self.rotate_center(CENTER_FRONT);
        self.rotate_by_start_and_direction(ROTATE_FRONT, DIRECTION_FRONT);
    }
    pub fn rotate_front_inv(&mut self) {
        self.rotate_center_inv(CENTER_FRONT);
        self.rotate_by_start_and_direction_inv(ROTATE_FRONT, DIRECTION_FRONT);
    }
    pub fn rotate_rigth(&mut self) {
        self.rotate_center(CENTER_RIGTH);
        self.rotate_by_start_and_direction(ROTATE_RIGTH, DIRECTION_RIGTH);
    }
    pub fn rotate_rigth_inv(&mut self) {
        self.rotate_center_inv(CENTER_RIGTH);
        self.rotate_by_start_and_direction_inv(ROTATE_RIGTH, DIRECTION_RIGTH);
    }
    pub fn rotate_up(&mut self) {
        self.rotate_center(CENTER_UP);
        self.rotate_by_start_and_direction(ROTATE_UP, DIRECTION_UP);
    }
    pub fn rotate_up_inv(&mut self) {
        self.rotate_center_inv(CENTER_UP);
        self.rotate_by_start_and_direction_inv(ROTATE_UP, DIRECTION_UP);
    }
    pub fn rotate_left(&mut self) {
        self.rotate_center(CENTER_LEFT);
        self.rotate_by_start_and_direction(ROTATE_LEFT, DIRECTION_LEFT);
    }
    pub fn rotate_left_inv(&mut self) {
        self.rotate_center_inv(CENTER_LEFT);
        self.rotate_by_start_and_direction_inv(ROTATE_LEFT, DIRECTION_LEFT);
    }
    pub fn rotate_down(&mut self) {
        self.rotate_center(CENTER_DOWN);
        self.rotate_by_start_and_direction(ROTATE_DOWN, DIRECTION_DOWN);
    }
    pub fn rotate_down_inv(&mut self) {
        self.rotate_center_inv(CENTER_DOWN);
        self.rotate_by_start_and_direction_inv(ROTATE_DOWN, DIRECTION_DOWN);
    }
    pub fn rotate_back(&mut self) {
        self.rotate_center(CENTER_BACK);
        self.rotate_by_start_and_direction(ROTATE_BACK, DIRECTION_BACK);
    }
    pub fn rotate_back_inv(&mut self) {
        self.rotate_center_inv(CENTER_BACK);
        self.rotate_by_start_and_direction_inv(ROTATE_BACK, DIRECTION_BACK);
    }
    pub fn rotate_middle(&mut self) {
        self.rotate_by_start_and_direction(ROTATE_MIDDLE, DIRECTION_MIDDLE);
    }
    pub fn rotate_middle_inv(&mut self) {
        self.rotate_by_start_and_direction_inv(ROTATE_MIDDLE, DIRECTION_MIDDLE);
    }
} 

const CENTER_FRONT: (usize, usize) = (4, 4);
const ROTATE_FRONT: [(isize, isize); 4] = [
    (2, 5), (3, 2), (6, 3), (5, 6)
];
const DIRECTION_FRONT: [(isize, isize); 4] = [
    (0, -1), (1, 0), (0, 1), (-1, 0)
];

const CENTER_RIGTH: (usize, usize) = (4, 7);
const ROTATE_RIGTH: [(isize, isize); 4] = [
    (5, 9), (0, 5), (3, 5), (6, 5)
];
const DIRECTION_RIGTH: [(isize, isize); 4] = [
    (-1, 0), (1, 0), (1, 0), (1, 0)
];

const CENTER_UP: (usize, usize) = (1, 4);
const ROTATE_UP: [(isize, isize); 4] = [
    (3, 0), (3, 3), (3, 6), (3, 9)
];
const DIRECTION_UP: [(isize, isize); 4] = [
    (0, 1); 4
];

const CENTER_LEFT: (usize, usize) = (4, 1);
const ROTATE_LEFT: [(isize, isize); 4] = [
    (6, 3), (3, 3), (0, 3), (5, 11)  
];
const DIRECTION_LEFT: [(isize, isize); 4] = [
    (1, 0), (1, 0), (1, 0), (-1, 0)
];

const CENTER_DOWN: (usize, usize) = (7, 4);
const ROTATE_DOWN: [(isize, isize); 4] = [
    (5, 9), (5, 6), (5, 3), (5, 0)
];
const DIRECTION_DOWN: [(isize, isize); 4] = [
    (0, 1); 4
];

const CENTER_BACK: (usize, usize) = (4, 10);
const ROTATE_BACK: [(isize, isize); 4] = [
    (3, 8), (8, 5), (5, 0), (0, 3)
];
const DIRECTION_BACK: [(isize, isize); 4] = [
    (1, 0), (0, -1), (-1, 0), (0, 1)
];

const ROTATE_MIDDLE: [(isize, isize); 4] = [
    (6, 4), (3, 4), (0, 4), (5, 10)
];
const DIRECTION_MIDDLE: [(isize, isize); 4] = [
    (1, 0), (1, 0), (1, 0), (-1, 0)
];

#[cfg(test)]
mod test {
    use super::*;
    
    // R2 D L2 B2 L2 U B2 D B2 U' L R' D R' B D' F L2 D F
    const SCRAMBLED_SCAN: [[char; 12]; 9] = [
        [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ','o','y','o',' ',' ',' ',' ',' ',' '],
        ['y','r','b','y','r','y','g','o','b','o','w','g'],
        ['w','o','g','o','b','w','b','r','g','r','g','r'],
        ['r','g','w','r','b','w','b','b','o','w','y','y'],
        [' ',' ',' ','g','y','r',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],
    ];
    
    #[test]
    fn check_default() {
        let cube = Cube::default();
        assert_eq!(cube, Cube::from_scan([
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
        ]));
    }

    #[test]
    fn check_from() {
        let cube = Cube::from("R2 D L2 B2 L2 U B2 D B2 U' L R' D R' B D' F L2 D F");
        assert_eq!(cube , Cube::from_scan(SCRAMBLED_SCAN));
    }

    #[test]
    fn check_rotate_front() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_front();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','g','b',' ',' ',' ',' ',' ',' '],
            ['y','r','g','r','o','y','o','o','b','o','w','g'],
            ['w','o','y','b','b','r','y','r','g','r','g','r'],
            ['r','g','r','w','w','y','o','b','o','w','y','y'],
            [' ',' ',' ','b','b','g',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],
        ]));
    }
    
    #[test]
    fn check_rotate_front_inv() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_front_inv();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','g','b','b',' ',' ',' ',' ',' ',' '],
            ['y','r','o','y','w','w','r','o','b','o','w','g'],
            ['w','o','y','r','b','b','y','r','g','r','g','r'],
            ['r','g','o','y','o','r','g','b','o','w','y','y'],
            [' ',' ',' ','b','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],    
        ]))
    }

    #[test]
    fn check_rotate_rigth() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_rigth();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','r','g','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','w',' ',' ',' ',' ',' ',' '],
            ['y','r','b','y','r','r','b','b','g','o','w','g'],
            ['w','o','g','o','b','o','b','r','o','w','g','r'],
            ['r','g','w','r','b','g','o','g','b','w','y','y'],
            [' ',' ',' ','g','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','o',' ',' ',' ',' ',' ',' '],
        ]))
    }

    #[test]
    fn check_rotate_right_inv() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_rigth_inv();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','o',' ',' ',' ',' ',' ',' '],
            ['y','r','b','y','r','w','b','g','o','g','w','g'],
            ['w','o','g','o','b','w','o','r','b','o','g','r'],
            ['r','g','w','r','b','o','g','b','b','r','y','y'],
            [' ',' ',' ','g','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','w',' ',' ',' ',' ',' ',' '],
        ]))
    }

    #[test]
    fn check_rotate_up() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_up();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','o','b','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','g',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','w','w',' ',' ',' ',' ',' ',' '],
            ['y','r','y','g','o','b','o','w','g','y','r','b'],
            ['w','o','g','o','b','w','b','r','g','r','g','r'],
            ['r','g','w','r','b','w','b','b','o','w','y','y'],
            [' ',' ',' ','g','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],
        ]));
    }

    #[test]
    fn check_rotate_up_inv() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_up_inv();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','w','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','g','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','r','b','o',' ',' ',' ',' ',' ',' '],
            ['o','w','g','y','r','b','y','r','y','g','o','b'],
            ['w','o','g','o','b','w','b','r','g','r','g','r'],
            ['r','g','w','r','b','w','b','b','o','w','y','y'],
            [' ',' ',' ','g','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],
        ]));
    }

    #[test]
    fn check_rotate_left() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_left();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','y','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','r','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','g','y','o',' ',' ',' ',' ',' ',' '],
            ['r','w','y','r','r','y','g','o','b','o','w','b'],
            ['g','o','r','b','b','w','b','r','g','r','g','y'],
            ['w','g','b','o','b','w','b','b','o','w','y','g'],
            [' ',' ',' ','y','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','r','o','g',' ',' ',' ',' ',' ',' '],
        ]));
    }
    
    #[test]
    fn check_rotate_left_inv() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_left_inv();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','y','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','r','y','o',' ',' ',' ',' ',' ',' '],
            ['b','g','w','g','r','y','g','o','b','o','w','o'],
            ['r','o','g','y','b','w','b','r','g','r','g','b'],
            ['y','w','r','b','b','w','b','b','o','w','y','r'],
            [' ',' ',' ','y','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','r','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','g','o','g',' ',' ',' ',' ',' ',' '],
        ]));
    }

    #[test]
    fn check_rotate_down() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_down();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','o',' ',' ',' ',' ',' ',' '],
            ['y','r','b','y','r','y','g','o','b','o','w','g'],
            ['w','o','g','o','b','w','b','r','g','r','g','r'],
            ['w','y','y','r','g','w','r','b','w','b','b','o'],
            [' ',' ',' ','b','y','g',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','w','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','g','o','r',' ',' ',' ',' ',' ',' '],
        ]));
    }

    #[test]
    fn check_rotate_down_inv() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_down_inv();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','o',' ',' ',' ',' ',' ',' '],
            ['y','r','b','y','r','y','g','o','b','o','w','g'],
            ['w','o','g','o','b','w','b','r','g','r','g','r'],
            ['r','b','w','b','b','o','w','y','y','r','g','w'],
            [' ',' ',' ','r','o','g',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','g','y','b',' ',' ',' ',' ',' ',' '],
        ]));
    }

    #[test]
    fn check_rotate_back() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_back();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','b','g','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','o',' ',' ',' ',' ',' ',' '],
            ['w','r','b','y','r','y','g','o','g','w','r','o'],
            ['g','o','g','o','b','w','b','r','o','y','g','w'],
            ['r','g','w','r','b','w','b','b','b','y','r','g'],
            [' ',' ',' ','g','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','r',' ',' ',' ',' ',' ',' '],
        ]));
    }

    #[test]
    fn check_rotate_back_inv() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_back_inv();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','r','w','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','o',' ',' ',' ',' ',' ',' '],
            ['b','r','b','y','r','y','g','o','r','g','r','y'],
            ['o','o','g','o','b','w','b','r','g','w','g','y'],
            ['g','g','w','r','b','w','b','b','w','o','r','w'],
            [' ',' ',' ','g','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','g','b',' ',' ',' ',' ',' ',' '],
        ]));
    }

    #[test]
    fn check_rotate_middle() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_middle();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','r','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','w','o',' ',' ',' ',' ',' ',' '],
            ['y','r','b','y','g','y','g','o','b','o','o','g'],
            ['w','o','g','o','y','w','b','r','g','r','w','r'],
            ['r','g','w','r','y','w','b','b','o','w','y','y'],
            [' ',' ',' ','g','r','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','b','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','b','g',' ',' ',' ',' ',' ',' '],
        ]));
    }

    #[test]
    fn check_rotate_middle_inv() {
        let mut cube_scrambled = Cube::from_scan(SCRAMBLED_SCAN);
        cube_scrambled.rotate_middle_inv();
        assert_eq!(cube_scrambled, Cube::from_scan([
            [' ',' ',' ','r','r','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','b','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','b','o',' ',' ',' ',' ',' ',' '],
            ['y','r','b','y','y','y','g','o','b','o','y','g'],
            ['w','o','g','o','w','w','b','r','g','r','y','r'],
            ['r','g','w','r','o','w','b','b','o','w','g','y'],
            [' ',' ',' ','g','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','g','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','w','g',' ',' ',' ',' ',' ',' '],
        ]));
    }
}
