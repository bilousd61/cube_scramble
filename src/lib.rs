#![allow(dead_code)]
use std::fmt;
use colored::*;

#[derive(PartialEq)]
pub struct Cube {
    pub scan: [[char; 12]; 9]
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "\n  0 2 4 6 8 10".to_string();
        let string = format!("{}", self);
        let result_chars: Vec<_> = string.lines().collect();
        for i in 1..result_chars.len() {
            result = format!("{}\n{} {}", result, i - 1, result_chars[i]);
        }
        return write!(f, "{}", result);
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "\n".to_string();
        for layer in self.scan {
            for letter in layer {
                let letter_string = letter.to_string();
                let letter_str = letter_string.trim();
                result = format!("{}{}", result, match letter_str {
                    "b" => letter_str.blue(),
                    "r" => letter_str.red(),
                    "y" => letter_str.truecolor(255, 255, 0),
                    "o" => letter_str.truecolor(255, 135, 0),
                    "g" => letter_str.green(),
                    "w" => letter_str.white(),
                    _ => " ".normal()
                })
            }
            result.push('\n');
        }
        return write!(f, "{}", result)
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube { scan: [
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
        ] }
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
            let _ = instr.reverse();
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
        let cube = Cube { ..Default::default() };
        assert_eq!(cube, Cube { scan: [
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','y',' ',' ',' ',' ',' ',' '],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            ['o','o','o','b','b','b','r','r','r','g','g','g'],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','w','w',' ',' ',' ',' ',' ',' '],
        ] });
    }

    #[test]
    fn check_rotate_front() {
        let mut cube_scrambled = Cube { scan: SCRAMBLED_SCAN };
        cube_scrambled.rotate_front();
        assert_eq!(cube_scrambled, Cube { scan: [
            [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','w','g','b',' ',' ',' ',' ',' ',' '],
            ['y','r','g','r','o','y','o','o','b','o','w','g'],
            ['w','o','y','b','b','r','y','r','g','r','g','r'],
            ['r','g','r','w','w','y','o','b','o','w','y','y'],
            [' ',' ',' ','b','b','g',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],
        ] });
    }
    
    #[test]
    fn check_rotate_front_inv() {
        let mut cube_scrambled = Cube { scan: SCRAMBLED_SCAN };
        cube_scrambled.rotate_front_inv();
        assert_eq!(cube_scrambled, Cube { scan: [
            [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','g','b','b',' ',' ',' ',' ',' ',' '],
            ['y','r','o','y','w','w','r','o','b','o','w','g'],
            ['w','o','y','r','b','b','y','r','g','r','g','r'],
            ['r','g','o','y','o','r','g','b','o','w','y','y'],
            [' ',' ',' ','b','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],    
        ] });
    }

    #[test]
    fn check_rotate_rigth() {
        let mut cube_scrambled = Cube { scan: SCRAMBLED_SCAN };
        cube_scrambled.rotate_rigth();
        assert_eq!(cube_scrambled, Cube { scan: [
            [' ',' ',' ','r','g','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','w',' ',' ',' ',' ',' ',' '],
            ['y','r','b','y','r','r','b','b','g','o','w','g'],
            ['w','o','g','o','b','o','b','r','o','w','g','r'],
            ['r','g','w','r','b','g','o','g','b','w','y','y'],
            [' ',' ',' ','g','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','o',' ',' ',' ',' ',' ',' '],
        ] })
    }

    #[test]
    fn check_rotate_right_inv() {
        let mut cube_scrambled = Cube { scan: SCRAMBLED_SCAN };
        cube_scrambled.rotate_rigth_inv();
        assert_eq!(cube_scrambled, Cube { scan: [
            [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','o',' ',' ',' ',' ',' ',' '],
            ['y','r','b','y','r','w','b','g','o','g','w','g'],
            ['w','o','g','o','b','w','o','r','b','o','g','r'],
            ['r','g','w','r','b','o','g','b','b','r','y','y'],
            [' ',' ',' ','g','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','w',' ',' ',' ',' ',' ',' '],
        ] })
    }

    #[test]
    fn check_rotate_up() {
        let mut cube_scrambled = Cube { scan: SCRAMBLED_SCAN };
        cube_scrambled.rotate_up();
        assert_eq!(cube_scrambled, Cube { scan: [
            [' ',' ',' ','o','b','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','y','g',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','w','w',' ',' ',' ',' ',' ',' '],
            ['y','r','y','g','o','b','o','w','g','y','r','b'],
            ['w','o','g','o','b','w','b','r','g','r','g','r'],
            ['r','g','w','r','b','w','b','b','o','w','y','y'],
            [' ',' ',' ','g','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],
        ] });
    }

    #[test]
    fn check_rotate_up_inv() {
        let mut cube_scrambled = Cube { scan: SCRAMBLED_SCAN };
        cube_scrambled.rotate_up_inv();
        assert_eq!(cube_scrambled, Cube { scan: [
            [' ',' ',' ','w','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','g','y','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','r','b','o',' ',' ',' ',' ',' ',' '],
            ['o','w','g','y','r','b','y','r','y','g','o','b'],
            ['w','o','g','o','b','w','b','r','g','r','g','r'],
            ['r','g','w','r','b','w','b','b','o','w','y','y'],
            [' ',' ',' ','g','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],
        ] });
    }

    #[test]
    fn check_rotate_left() {
        let mut cube_scrambled = Cube { scan: SCRAMBLED_SCAN };
        cube_scrambled.rotate_left();
        assert_eq!(cube_scrambled, Cube { scan: [
            [' ',' ',' ','y','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','r','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','g','y','o',' ',' ',' ',' ',' ',' '],
            ['r','w','y','r','r','y','g','o','b','o','w','b'],
            ['g','o','r','b','b','w','b','r','g','r','g','y'],
            ['w','g','b','o','b','w','b','b','o','w','y','g'],
            [' ',' ',' ','y','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','w','o',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','r','o','g',' ',' ',' ',' ',' ',' '],
        ] });
    }
}
