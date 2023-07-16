#![allow(dead_code)]
use std::fmt;
use colored::*;

#[derive(PartialEq)]
struct Cube {
    scan: [[char; 12]; 9]
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
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
    fn rotate_front(&mut self) {
        self.rotate_center(CENTER_FRONT);
        for i in 0..=2 {
            let mut instr = [(0, 0); 4];
            for j in 0..=3 {
                let rotate = ROTATE_FRONT[j];
                let direction = DIRECTION_FRONT[j];
                instr[j] = (
                    (rotate.0 + direction.0 * i as isize) as usize, 
                    (rotate.1 + direction.1 * i as isize) as usize);
            }
            self.rotate_by_instr(instr);
        }
    }
    fn rotate_front_inv(&mut self) {
        self.rotate_center_inv(CENTER_FRONT);
        let mut rotate_front_inv = ROTATE_FRONT;
        rotate_front_inv.reverse();
        let mut direction_front_inv = DIRECTION_FRONT;
        direction_front_inv.reverse();
        for i in 0..=2 {
            let mut instr = [(0, 0); 4];
            for j in 0..=3 {
                let rotate = rotate_front_inv[j];
                let direction = direction_front_inv[j];
                instr[j] = (
                    (rotate.0 + direction.0 * i as isize) as usize, 
                    (rotate.1 + direction.1 * i as isize) as usize);
            }
            self.rotate_by_instr(instr);
        }
    }
    /*fn rotate_front_inv(&mut self) {
        self.rotate_center_inv(CENTER_FRONT);
        for mut step in ROTATE_FRONT {
            let _ = step.reverse();
            self.rotate_by_instr(step);
        }
    }*/
    /*fn rotate_rigth(&mut self) {
        self.rotate_center(CENTER_RIGTH);
        
    }*/
}

const CENTER_FRONT: (usize, usize) = (4, 4);
const ROTATE_FRONT: [(isize, isize); 4] = [
    (2, 5), (3, 2), (6, 3), (5, 6)
];
const DIRECTION_FRONT: [(isize, isize); 4] = [
    (0, -1), (1, 0), (0, 1), (-1, 0)
];
const CENTER_RIGTH: (usize, usize) = (4, 7);
const ROTATE_RIGTH: [[(usize, usize); 4]; 3] = [
    [(5, 9), (6, 5), (3, 5), (0, 5)],
    [(4, 9), (7, 5), (4, 5), (1, 5)],
    [(3, 9), (8, 5), (5, 5), (2, 5)],
];

#[cfg(test)]
mod test {
    use super::*;
    
    const SCRAMBLED_SCAN: [[char; 12]; 9] = [
            [' ',' ',' ','r','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','y','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','o','y','o',' ',' ',' ',' ',' ',' '],
            ['y','o','b','y','r','y','g','o','b','o','w','g'],
            ['w','o','g','o','b','w','b','r','g','r','g','r'],
            ['r','g','w','r','b','w','b','b','o','w','y','y'],
            [' ',' ',' ','g','y','r',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','y',' ',' ',' ',' ',' ',' '],
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
            ['y','o','g','r','o','y','o','o','b','o','w','g'],
            ['w','o','y','b','b','r','y','r','g','r','g','r'],
            ['r','g','r','w','w','y','o','b','o','w','y','y'],
            [' ',' ',' ','b','b','g',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','y',' ',' ',' ',' ',' ',' '],
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
            ['y','o','o','y','w','w','r','o','b','o','w','g'],
            ['w','o','y','r','b','b','y','r','g','r','g','r'],
            ['r','g','o','y','o','r','g','b','o','w','y','y'],
            [' ',' ',' ','b','g','w',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','y','w','y',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ','b','o','g',' ',' ',' ',' ',' ',' '],    
        ] });
    }
}
