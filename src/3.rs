// --- Day 3: Crossed Wires ---
//
// https://adventofcode.com/2019/day/3
//
use std::collections::HashSet;

type Point = (i32, i32);
type TracePath = Vec<Point>;
type Direction = (char, usize);
type Directions = Vec<Direction>;


fn trace_path(directions: Directions) -> TracePath {
    struct Pos { pub x: i32, pub y: i32, }
    let mut trace = Vec::new();
    let mut pos = Pos{x:0, y:0};
    for (d, steps) in directions {
        match d {
            'R' => {
                for _ in 0..steps {
                    pos.x += 1;
                    trace.push((pos.x, pos.y));
                }
            }
            'L' => {
                for _ in 0..steps {
                    pos.x -= 1;
                    trace.push((pos.x, pos.y));
                }
            }
            'U' => {
                for _ in 0..steps {
                    pos.y += 1;
                    trace.push((pos.x, pos.y));
                }
            }
            'D' => {
                for _ in 0..steps {
                    pos.y -= 1;
                    trace.push((pos.x, pos.y));
                }
            }
            _ => unreachable!(),
        }
    }

    trace
}

fn steps_to_intersection(tp: &TracePath, inter: &Point) -> Option<i32> {
    for (idx, point) in tp.iter().enumerate() {
        if point == inter {
            return Some(idx as i32 + 1 as i32);
        }
    }
    None
}

fn get_intersections(p1: &Vec<(i32, i32)>, p2: &Vec<(i32, i32)>) -> Vec<Point> {
    let h1: HashSet<(i32, i32)> = p1.iter().copied().collect();
    let h2: HashSet<(i32, i32)> = p2.iter().copied().collect();
    h1.intersection(&h2).copied().collect::<Vec<_>>()
}

fn mdist(a:  Point, b: Point) -> i32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs())
}

fn manhattan_dist(p1: Directions, p2: Directions) -> i32 {
    get_intersections(&trace_path(p1), &trace_path(p2))
        .iter()
        .map(|x| mdist(*x, (0, 0)))
        .min()
        .unwrap()
}

fn minimal_steps(p1: Directions, p2: Directions) -> i32 {
    let tp1 = trace_path(p1);
    let tp2 = trace_path(p2);
    let intersections = get_intersections(&tp1, &tp2);
    let mut steps = vec![];
    for x in intersections {
        steps.push(
            steps_to_intersection(&tp1, &x).unwrap() + steps_to_intersection(&tp2, &x).unwrap(),
        );
    }

    *steps.iter().min().unwrap()
}

fn parse_directions(d: Vec<&str>) -> Vec<Direction> {
    d.iter()
        .map(|x| (x.chars().nth(0).unwrap(), x[1..].parse().unwrap()))
        .collect()
}

fn main() {
    let paths: Vec<Directions> = include_str!("input3.txt")
        .lines()
        .map(|l| l.split(',').collect())
        .map(|x| parse_directions(x))
        .collect();
    println!(
        "Part One: {:?}",
        manhattan_dist(paths[0].clone(), paths[1].clone())
    );
    println!(
        "Part Two: {:?}",
        minimal_steps(paths[0].clone(), paths[1].clone())
    );
}

#[test]
fn test_manhattan_dist() {
    let p1 = parse_directions(vec![
        "R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72",
    ]);
    let p2 = parse_directions(vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]);
    let p3 = parse_directions(vec![
        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
    ]);
    let p4 = parse_directions(vec![
        "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
    ]);
    assert_eq!(159, manhattan_dist(p1, p2));
    assert_eq!(135, manhattan_dist(p3, p4));
}

#[test]
fn test_minimal_steps() {
    let p1 = parse_directions(vec![
        "R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72",
    ]);
    let p2 = parse_directions(vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]);
    let p3 = parse_directions(vec![
        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
    ]);
    let p4 = parse_directions(vec![
        "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
    ]);
    assert_eq!(610, minimal_steps(p1, p2));
    assert_eq!(410, minimal_steps(p3, p4));
}
