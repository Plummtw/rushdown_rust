use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

use std::collections::{HashMap, VecDeque};

const X: i8 = 6;
const Y: i8 = 6;

const H: char = 'H';
const V: char = 'V';

const U: char = 'U';
const D: char = 'D';
const L: char = 'L';
const R: char = 'R';

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Car
{
    pub id: u8,
    pub x:  i8,
    pub y:  i8,
    pub len: i8,
    pub dir: char
}

impl Car {
    pub fn parse_car(s: &str) -> Self {
        let arr = s.trim().split_whitespace().collect::<Vec<_>>();
        let id = arr[0];
        let x = arr[1];
        let y = arr[2];
        let len = arr[3];
        let dir = arr[4];
        Car {
            id: id.parse::<u8>().unwrap(),
            x: x.parse::<i8>().unwrap(),
            y: y.parse::<i8>().unwrap(),
            len: len.parse::<i8>().unwrap(),
            dir: dir.chars().next().unwrap(),
        }
    }

    pub fn contains(&self, x: i8, y: i8) -> bool {
        if self.dir == V {
            (self.x == x) && (y >= self.y) && (y < self.y + self.len)
        } else if self.dir == H {
            (self.y == y) && (x >= self.x) && (x < self.x + self.len)
        } else {
            false
        }
    }

    pub fn can_move(&self, move_dir: char, cars: &Vec<Car>) -> bool {
        let cars = cars.iter().filter(|car| car.id != self.id).collect::<Vec<_>>();
        match move_dir {
            U => (self.dir == V) && block_valid(self.x, self.y-1, cars),
            D => (self.dir == V) && block_valid(self.x, self.y+self.len, cars),
            L => (self.dir == H) && block_valid(self.x-1, self.y, cars),
            R => (self.dir == H) && block_valid(self.x+self.len, self.y, cars),
            _ => false
        }
    }

    pub fn car_move(&mut self, move_dir: char) {
        match move_dir {
            U => self.y -=1 ,
            D => self.y += 1,
            L => self.x -= 1,
            R => self.x += 1,
            _ => {}
        }
    }
}

pub fn block_valid(x: i8, y: i8, cars: Vec<&Car>) -> bool {
    if !((x >= 0) && (x < X) && (y >= 0) && (y < Y)) {
        return false;
    }
    for car in cars {
        if car.contains(x, y) {
            return false;
        }
    }
    true
}

pub fn all_moves() -> Vec<char> {
    vec![U, D, L, R]
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CarMove {
    car: Car,
    move_dir: char,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct State {
  pub cars: Vec<Car>,
  pub history: Vec<CarMove>,
  pub next_move: CarMove,
}

impl State {
    pub fn parse_cars(car_strs: Vec<&str>) -> State {
        let cars = car_strs.iter().map(|&car_str| Car::parse_car(car_str)).collect::<Vec<_>>();
        State {
            cars: cars,
            history: Vec::new(),
            next_move: CarMove { // Temporary value (Not useds)
                car: Car {id:0, x:0, y:0, len:0, dir:H },
                move_dir: H
            }
        }
    }

    pub fn all_valid_moves(&self) -> Vec<CarMove> {
        let mut result: Vec<CarMove> = Vec::new();
        for car in &(self.cars) {
            for move_dir in all_moves() {
                if car.can_move(move_dir, &self.cars) {
                    result.push(CarMove {car: car.clone(), move_dir: move_dir});
                }
            }
        }
        result
    }

    pub fn moves_to_states(&self, car_moves: Vec<CarMove>) -> VecDeque<Self> {
        car_moves.iter().map(|car_move|
          State {
            cars: self.cars.clone(), 
            history: self.history.clone(),
            next_move: car_move.clone(),
          }
        ).collect::<VecDeque<_>>()
    }

    pub fn apply_move(&mut self, car_move: CarMove) {
        let car = self.cars.iter_mut().find(|car| car.id == car_move.car.id).unwrap();
        car.car_move(car_move.move_dir);
        self.history.push(car_move);
    }

    pub fn initial_states(&self) -> VecDeque<Self> {
        heueristics(VecDeque::new(), self.moves_to_states(self.all_valid_moves()))
    }

    pub fn generate_new_states(&self, old_states: VecDeque<Self>) -> VecDeque<Self> {
        heueristics(old_states, self.moves_to_states(self.all_valid_moves()))
    }
}

pub fn heueristics(old_states: VecDeque<State>, new_states: VecDeque<State>) -> VecDeque<State> {
    let mut old_states = old_states;
    let mut temp_states: VecDeque<State> = VecDeque::new();
    let mut new_states = new_states;

    while let Some(state) = new_states.pop_front() {
        if state.next_move.car.dir == V {
            old_states.push_back(state);
        } else {
            temp_states.push_back(state);
        }
    }

    while let Some(state) = temp_states.pop_front() {
        if state.next_move.car.dir == H && state.next_move.car.id != 0 && state.next_move.move_dir == L {
            old_states.push_front(state);
        } else {
            new_states.push_back(state);
        }
    }

    while let Some(state) = new_states.pop_front() {
        if state.next_move.car.id != 0 && state.next_move.move_dir == R {
            old_states.push_front(state);
        } else {
            temp_states.push_back(state);
        }
    }

    while let Some(car_move) = temp_states.pop_front() {
        old_states.push_back(car_move);
    }

    old_states
}

pub fn dfs(state: State, limit: usize) -> Option<State> {
    let mut global_history: HashMap<Vec<Car>, usize> = HashMap::new();
    let mut states = state.initial_states();

    while let Some(mut state) = states.pop_front() {
        // println!("Move: {:?}", state.next_move);
        state.apply_move(state.next_move.clone());
        // println!("{:?}", state.cars);

        let history_count = state.history.len();
        match global_history.get(&state.cars) {
            Some(history_size) => {
                // Check History Size ....
                if *history_size <= history_count {
                    continue;
                }
            },
            None => {}
        }

        let car0 = state.cars.iter().find(|car| car.id == 0).unwrap();
        if car0.x == 4 {
            return Some(state);
        }

        global_history.insert(state.cars.clone(), history_count);

        if history_count < limit {
            states = state.generate_new_states(states);
        }
    }
    None
}

pub fn print_history(car_move: &CarMove) {
    let dir = match car_move.move_dir {
        U => "UP",
        D => "DOWN",
        L => "LEFT",
        R => "RIGHT",
        _ => ""
    };
    println!("{} {}", car_move.car.id, dir);
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize); // Number of vehicles
    let mut first_loop = true;
    let mut input_strs: Vec<String> = Vec::with_capacity(n);

    let mut result: Option<State> = None;
    let mut rounds = 0usize;
    // game loop
    loop {
        
        for _ in 0..n as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            if first_loop {
                input_strs.push(input_line);
            }
        }

        if first_loop {
            let init_state = State::parse_cars(input_strs.iter().map(|s| s as &str).collect());
            result = dfs(init_state, 100);
        }

        match &result {
            Some(state) => {
                print_history(&state.history[rounds])
            },
            None => {},
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // println!("0 RIGHT"); // ID DIRECTION
        first_loop = false;
        rounds += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn test_dfs(cars: Vec<&str>) {
        let state = State::parse_cars(cars);
        match dfs(state, 4) {
            Some(state) => {
                println!("{:?}", state);
                for history in state.history {
                    println!("{:?}", history);
                }
            },
            None => println!("nil")
        }
    }

    #[test]
    fn test0() {
        let cars = vec!["0 1 2 2 H","3 4 2 2 V","13 0 4 3 H"];
        test_dfs(cars);
    }
}
