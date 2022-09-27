use std::collections::{HashMap, VecDeque};

const X: i8 = 6;
const Y: i8 = 6;

const H: u8 = 'H' as u8;
const V: u8 = 'V' as u8;

const U: u8 = 'U' as u8;
const D: u8 = 'D' as u8;
const L: u8 = 'L' as u8;
const R: u8 = 'R' as u8;

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub struct Car
{
    pub id: u8,
    pub x:  i8,
    pub y:  i8,
    pub len: i8,
    pub dir: u8
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
            dir: dir.bytes().next().unwrap(),
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

    pub fn can_move(&self, move_dir: u8, cars: Vec<Car>) -> bool {
        let cars = cars.iter().filter(|car| car.id != self.id).collect::<Vec<_>>();
        match move_dir {
            U => (self.dir == H) && block_valid(self.x, self.y-1, cars),
            D => (self.dir == H) && block_valid(self.x, self.y+self.len, cars),
            L => (self.dir == H) && block_valid(self.x-1, self.y, cars),
            R => (self.dir == H) && block_valid(self.x+self.len, self.y, cars),
            _ => false
        }
    }

    pub fn car_move(&mut self, move_dir: u8) {
        match move_dir {
            U => self.y -=1 ,
            D => self.y += 1,
            L => self.x -= 1,
            R => self.y += 1,
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

pub fn all_moves() -> Vec<u8> {
    vec![U, D, L, R]
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct CarMove {
    car: Car,
    move_dir: u8,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct State {
  pub cars: Vec<Car>,
  pub history: Vec<CarMove>,
  pub next_move: CarMove,
}

impl State {
    pub fn parse_cars(car_strs: Vec<&str>) -> Self {
        State {
            cars: car_strs.iter().map(|&car_str| Car::parse_car(car_str)).collect::<Vec<_>>(),
            history: Vec::new(),
            next_move: 0,
        }
    }

    pub fn all_valid_moves(&self) -> Vec<CarMove> {
        let mut result: Vec<CarMove> = Vec::new();
        for car in &(self.cars) {
            for move_dir in all_moves() {
                if car.can_move(move_dir, self.cars) {
                    result.push(CarMove {car: car.clone(), move_dir: move_dir});
                }
            }
        }
        result
    }

    pub fn moves_to_states(&self, car_moves: Vec<CarMove>) -> VecDeque<State> {
        car_moves.iter().map(|car_move|
          State {
            cars: self.cars.clone(), 
            history: self.history.clone(),
            next_move: car_move.clone(),
          }
        ).collect::<VecDeque<_>>()
    }

    pub fn apply_move(&mut self, car_move: CarMove) {
        let car = self.cars.iter_mut().find(|&car| car.id == car_move.car.id).unwrap();
        car.car_move(car_move.move_dir);
        self.history.push(car_move);
    }

    pub fn generate_new_states(&self) -> VecDeque<State> {
        heueristics(VecDeque::new(), self.moves_to_states(self.all_valid_moves()))
    }
}

pub fn heueristics(old_states: VecDeque<State>, new_states: VecDeque<State>) -> VecDeque<State> {
    let mut old_states = old_states;
    let mut temp_states: VecDeque<State> = VecDeque::new();
    let mut new_states = new_states;

    while let Some(state) = old_states.pop_front() {
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

pub fn dfs(state: State, limit: u8) -> Option<State> {
    let mut global_history: HashMap<Vec<Car>, u8> = HashMap::new();
    let mut states = state.generate_new_states();

    while states.len() > 0 {
        
    }
    None
}


fn main() {
    
    println!("Hello, world!");
}
