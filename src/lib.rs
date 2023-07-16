use std::fmt;

#[derive(PartialEq)]
struct Cube {
    scan: [[char; 12]; 9]
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "\n".to_string();
        for layer in self.scan {
            for letter in layer {
                result.push(letter)
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

trait Print {
    fn print(&self);
}

trait Rotate {
    fn rotate_by_instr(&mut self, instr: [(usize, usize); 4]);
    fn rotate_center(&mut self, center: (usize, usize));
    fn rotate_center_inv(&mut self, center: (usize, usize));
    fn rotate_front(&mut self);
    fn rotate_front_inv(&mut self);
}

impl Print for Cube {
    fn print(&self) {
        for layer in self.scan {
            for letter in layer {
                print!("{}", letter)
            }
            println!("");
        }
    }
}

fn instr_by_center(center: (usize, usize)) -> [[(usize, usize); 4]; 2] {
    let x = center.0;
    let y = center.1;
    [[(x - 1, y + 1), (x - 1, y - 1), (x + 1, y - 1), (x + 1, y + 1)], 
    [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]]
}

impl Rotate for Cube {
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
        for step in ROTATE_FRONT {
            self.rotate_by_instr(step);
        }
    }
    fn rotate_front_inv(&mut self) {
        self.rotate_center_inv(CENTER_FRONT);
        for mut step in ROTATE_FRONT {
            let _ = step.reverse();
            self.rotate_by_instr(step);
        }
    }
}

const CENTER_FRONT: (usize, usize) = (4, 4);
const ROTATE_FRONT: [[(usize, usize); 4]; 3] = [
    [(2, 5), (3, 2), (6, 3), (5, 6)], 
    [(2, 4), (4, 2), (6, 4), (4, 6)], 
    [(2, 3), (5, 2), (6, 5), (3, 6)],
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
    fn rotate_front() {
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
    fn rotate_front_inv() {
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
