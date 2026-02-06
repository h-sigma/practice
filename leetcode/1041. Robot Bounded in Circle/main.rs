#[derive(PartialEq)]
enum Dir {
    North,
    East,
    West,
    South
}

struct State {
    pub x: i32,
    pub y: i32,
    pub dir: Dir,
}

impl State {
    pub fn new() -> State {
        State {
            x: 0,
            y: 0,
            dir: Dir::North
        }
    }

    pub fn forward(self) -> State {
        match self.dir {
            Dir::North => State {
                y: self.y + 1,
                ..self
            },
            Dir::East => State {
                x: self.x + 1,
                ..self
            },
            Dir::West => State {
                x: self.x - 1,
                ..self
            },
            Dir::South => State {
                y: self.y - 1,
                ..self
            }
        }
    }

    pub fn left(self) -> State {
        State {
            dir: match self.dir {
                Dir::North => Dir::West,
                Dir::East => Dir::North,
                Dir::South => Dir::East,
                Dir::West => Dir::South,
            },
            ..self
        }
    }


    pub fn right(self) -> State {
        State {
            dir: match self.dir {
                Dir::North => Dir::East,
                Dir::East => Dir::South,
                Dir::South => Dir::West,
                Dir::West => Dir::North,
            },
            ..self
        }
    }
}

impl Solution {
    // robot is going to be bounded if:
    // 1. it returns to origin, no matter its final rotation
    // because all directions are symmetrical
    // 2. it is facing any direction other than north at the end
    // because if it's rotated by 90deg either way, it returns to its original position in 3 turns, and if it's rotated by 180deg, it returns to its original position in 1 turn
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut state = State::new();
        for instruction in instructions.chars() {
            state = match instruction {
                'G' => state.forward(),
                'L' => state.left(),
                'R' => state.right(),
                _ => state,
            };
        }
        if state.x == 0 && state.y == 0 {
            return true;
        }
        if state.dir == Dir::North {
            return false;
        }

        return true;
    }
}