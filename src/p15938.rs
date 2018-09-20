use std;
use std::io::BufRead;
use std::str::FromStr;
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

#[derive(Debug)]
enum Direction {
    NONE,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Step {
    pos: Point,
    from: Direction,
}

struct Data {
    sw_pos: Point,
    target_pos: Point,
    obstacles: std::collections::HashSet<Point>,
    time_limit: u32,
}

pub fn solve() {
    let data = process_input();
    let mut success_sum = 0;
    let mut cur_positions = vec![Step { pos: data.sw_pos.clone(), from: Direction::NONE }];
    for i in 0..data.time_limit {
        let (new_poses, success_count) = go_forward(&cur_positions, &data.target_pos, &data.obstacles);
        success_sum += success_count;
        cur_positions = new_poses;
    }
    println!("{}", success_sum % (10u32.pow(9) + 7))
}

fn process_input() -> Data {
    let sin = std::io::stdin();
    let mut handle = sin.lock();
    let sw_pos = get_position(&mut handle);
    let time_limit = get_time_limit(&mut handle);
    let target_pos = get_position(&mut handle);
    let obstacles = get_obstacles(&mut handle);
    Data {
        sw_pos,
        target_pos,
        time_limit,
        obstacles,
    }
}

fn get_position(handle: &mut std::io::StdinLock) -> Point {
    let mut line = String::new();
    handle.read_line(&mut line);
    let mut splited_iter = line.split_whitespace();
    Point {
        x: i32::from_str(splited_iter.next().expect("no number")).expect("wrong number"),
        y: i32::from_str(splited_iter.next().expect("no number")).expect("wrong number"),
    }
}

fn get_time_limit(handle: &mut std::io::StdinLock) -> u32 {
    let mut line = String::new();
    handle.read_line(&mut line);
    u32::from_str(line.trim()).expect("wrong number")
}

fn get_obstacles(handle: &mut std::io::StdinLock) -> std::collections::HashSet<Point> {
    let mut line = String::new();
    handle.read_line(&mut line);
    let obstacle_num = u32::from_str(line.trim()).expect("wrong number");
    (0..obstacle_num).map(|_| {
        let mut line = String::new();
        handle.read_line(&mut line);
        line
    }).map(|x| {
        let mut coord_iter = x.split_whitespace().map(|n| i32::from_str(n).expect("wrong number"));
        Point{
            x: coord_iter.next().unwrap(),
            y: coord_iter.next().unwrap()
        }
    }).collect()
}

fn go_forward(cur_pos: &Vec<Step>, target: &Point, obstacles: &std::collections::HashSet<Point>) -> (Vec<Step>, u32) {
    let new_pos: Vec<Step> = one_step_from_now(cur_pos, obstacles);
    let (success_pos, new_pos): (Vec<Step>, Vec<Step>) = new_pos.into_iter().partition(|x| x.pos == *target);
    let success_count = success_pos.iter().fold(0u32, |sum, _| sum + 1);
    (new_pos, success_count)
}

fn one_step_from_now(pos: &Vec<Step>, obstacles: &std::collections::HashSet<Point>) -> Vec<Step> {
    pos.into_iter().flat_map(|x| {
        let new_pos = vec![
            Step {
                pos: Point { x: x.pos.x + 1, y: x.pos.y },
                from: Direction::LEFT,
            },
            Step {
                pos: Point { x: x.pos.x - 1, y: x.pos.y },
                from: Direction::RIGHT,
            },
            Step {
                pos: Point { x: x.pos.x, y: x.pos.y + 1 },
                from: Direction::DOWN,
            },
            Step {
                pos: Point { x: x.pos.x, y: x.pos.y - 1 },
                from: Direction::UP,
            }
        ];
        new_pos.into_iter().filter(|np| validate_step(np, x, obstacles)).collect::<Vec<Step>>()
    }).collect()
}

fn validate_step(step: &Step, old_step: &Step, obstacles: &std::collections::HashSet<Point>) -> bool {
    check_obstacles(step, obstacles) && check_direction(step, old_step)
}

fn check_obstacles(step: &Step, obstacles: &std::collections::HashSet<Point>) -> bool {
    !obstacles.contains(&step.pos)
}

fn check_direction(step: &Step, old_step: &Step) -> bool {
    use self::Direction::*;
    match (&step.from, &old_step.from) {
        (LEFT, RIGHT) | (RIGHT, LEFT) | (UP, DOWN) | (DOWN, UP) => false,
        _ => true
    }
}