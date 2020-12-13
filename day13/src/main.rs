use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;

pub fn get_input() -> String {
    let mut buf = String::new();
    let mut f = File::open("input.txt").expect("failed to open file");
    f.read_to_string(&mut buf).expect("failed to read contents of file");
    buf
}

#[derive(Eq, Debug)]
struct Shuttle {
    id: i64,
    next_departure_time: i64,
    next_departure_wait: i64
}

impl Shuttle {
    pub fn print(&self) {
        println!("Shuttle id: [{:3}] next departure: [{:4}], next departure in wait time: [{:3}]", self.id, self.next_departure_time, self.next_departure_wait);
    }
    pub fn answer(&self) -> i64 { self.next_departure_wait * self.id } 
}

impl Ord for Shuttle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.next_departure_wait.cmp(&other.next_departure_wait)
    }
}

impl PartialOrd for Shuttle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Shuttle {
    fn eq(&self, other: &Self) -> bool {
        self.next_departure_wait == other.next_departure_wait
    }
}


// God I love Rust.
fn main() {
    let data = get_input();
    let lines: Vec<&str> = data.lines().collect();
    let time_stamp = lines[0].parse::<i64>().expect("failed to parse time stamp");
    let bus_id_data = lines[1];
    let bus_ids: Vec<i64> = bus_id_data.split(",")
                             .filter_map(|str| {
                                str.parse::<i64>().ok() // ok converts to Option<T> and filter_map filters out any None's, thus any non-numbers get filtered
                             })
                             .collect();

    let mut shuttles: Vec<Shuttle> = bus_ids.iter().filter_map(|id| {
        let res = (time_stamp as f64 / *id as f64).floor() as i64;
        if (res * id) < time_stamp {
            let t = res * id;
            let m = id - (time_stamp % id);
            let next_departure_time = time_stamp + m;
            let next_departure_wait = next_departure_time - time_stamp;
            Some(Shuttle { id: *id, next_departure_time, next_departure_wait })
        } else {
            None
        }
    }).collect();

    println!("Sorting shuttles by how long the wait is in time units...\n");
    shuttles.sort();
 
    for shuttle in &shuttles {
        shuttle.print();
    }

    println!("Result of part 1: {}", shuttles[0].answer())
}
