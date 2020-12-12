use std::fs::{File};
use std::io::Read;

#[derive(Debug)]
pub enum MoveAction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32)
}

#[derive(Debug)]
pub struct Waypoint {
    x: i32,
    y: i32,
}

pub enum Rotation {
    CounterClockWise(i32),
    ClockWise(i32)
}

impl Waypoint {
    pub fn apply_rotation_mat(&mut self, degrees: f64) {
        let rad = degrees * std::f64::consts::PI / 180f64;
        let newx = ((self.x as f64 * rad.cos()) - (self.y as f64 * rad.sin())).round();
        let newy = ((self.x as f64 * rad.sin()) + (self.y as f64 * rad.cos())).round();
        self.x = newx as i32;
        self.y = newy as i32;
    }
}

pub struct MoveActionList {
    pub actions: Vec<MoveAction>
}

pub struct Ship {
    x: i32,
    y: i32,
    waypoint: Waypoint,
}


impl Ship {
    pub fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            waypoint: Waypoint { x: 10, y: 1 } 
        }
    }

    pub fn exec(&mut self, action: MoveAction) {
        match action {
            MoveAction::North(n) => self.waypoint.y += n,
            MoveAction::East(n) => self.waypoint.x += n,
            MoveAction::South(n) => self.waypoint.y -= n,
            MoveAction::West(n) => self.waypoint.x -= n,
            MoveAction::Left(deg) => {
                let degrees = deg as f64;
                self.waypoint.apply_rotation_mat(degrees);
            },
            MoveAction::Right(deg) => {
                let degrees = -deg as f64;
                self.waypoint.apply_rotation_mat(degrees);
            }
            MoveAction::Forward(steps) => {
                self.x = self.x + (steps * self.waypoint.x);
                self.y = self.y + (steps * self.waypoint.y);
            }
        }
    }

    pub fn exec_all(&mut self, actions: MoveActionList) {
        for a in actions.actions {
            self.exec(a);
        }
    }

    pub fn absolute_position(&self) -> (i64, i64) {
        (self.x as i64, self.y as i64)
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() as i32 + self.y.abs() as i32
    }
}

/// We assume that our input is _perfect_ - literally nothing can go wrong, otherwise KABOOM
impl From<String> for MoveActionList {
    fn from(data: String) -> Self {
        let actions = data.lines().map(|l| {
             match &l[0..1]  {
                "N" => MoveAction::North(l[1..].parse::<i32>().expect("Failed to parse input")),
                "E" => MoveAction::East(l[1..].parse::<i32>().expect("Failed to parse input")),
                "S" => MoveAction::South(l[1..].parse::<i32>().expect("Failed to parse input")),
                "W" => MoveAction::West(l[1..].parse::<i32>().expect("Failed to parse input")),
                "F" => MoveAction::Forward(l[1..].parse::<i32>().expect("Failed to parse input")),
                "R" => MoveAction::Right(l[1..].parse::<i32>().expect("Failed to parse input")),
                "L" => MoveAction::Left(l[1..].parse::<i32>().expect("Failed to parse input")),
                _ => panic!("ERRONEOUS INPUT. KABOOM")
        }}).collect();

        MoveActionList { actions }
    }
}

pub fn some_input() -> String {
    let mut f = File::open("input.txt").expect("Failed to open input file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("Failed to read file contents");
    buf
}

fn main() {
    let mut ship = Ship::new();
    let input = some_input();
    let actions = MoveActionList::from(input);
    ship.exec_all(actions);
    let abs_position = ship.absolute_position();
    let man_dist = ship.manhattan_distance();
    println!("Absolute position: {:?} - Manhattan distance is: {}", abs_position, man_dist);
}
