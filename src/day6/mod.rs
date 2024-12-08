use crate::grid::Grid;
use ahash::AHashSet;
use rayon::prelude::*;
use itertools::Itertools;

type Point = (usize, usize);

#[inline]
pub fn part1(input: &str) -> usize {
    let (grid, initial_position) = parse(input);
    num_steps_to_leave_the_grid(&grid, initial_position, None).unwrap()
}

fn num_steps_to_leave_the_grid(grid: &Grid, initial_position: Point, obstruction: Option<Point>) -> Option<usize> {
    let mut guard_direction = Direction::Up;
    let mut guard_position = initial_position;
    let mut visited = AHashSet::with_capacity(grid.rows * grid.columns);
    let mut visited_with_direction = AHashSet::with_capacity(grid.rows * grid.columns);
    while let Some((next_position, next_direction)) = next_valid_position(&grid, obstruction, guard_position, guard_direction) {
        visited.insert(guard_position);
        let position_with_direction = (guard_position.0, guard_position.1, guard_direction);
        if visited_with_direction.contains(&position_with_direction) {
            // We're in a loop, quit.
            return None;
        }
        visited_with_direction.insert(position_with_direction);
        guard_position = next_position;
        guard_direction = next_direction;
    }
    Some(visited.len() + 1)
}

fn next_valid_position(grid: &Grid, obstruction: Option<Point>, position: Point, mut direction: Direction) -> Option<(Point, Direction)> {
    while let Some(next_position) = direction.next_position(grid, position) {
        if obstruction.is_some() && obstruction.unwrap() == next_position {
            direction = direction.turn_right();
            continue;
        }
        match grid.get(next_position) {
            Some('.') => {
                return Some((next_position, direction));
            }
            Some('#') => direction = direction.turn_right(),
            _ => unreachable!("invalid character"),
        }
    }
    // Left the grid
    None
}

#[inline]
pub fn part2(input: &str) -> usize {
    let (grid, initial_position) = parse(input);

    (0..grid.rows).cartesian_product(0..grid.columns)
        .map(|obstruction| num_steps_to_leave_the_grid(&grid, initial_position, Some(obstruction)))
        .filter(|optional_steps| optional_steps.is_none())
        .count()
}



#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!("No other direction supported"),
        }
    }
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }

    fn next_position(&self, grid: &Grid, position: Point) -> Option<Point> {
        match self {
            Direction::Up => {
                if position.0 == 0 {
                    return None;
                }
                Some((position.0 - 1, position.1))
            }
            Direction::Right => {
                if position.1 + 1 == grid.columns {
                    return None;
                }
                Some((position.0, position.1 + 1))
            }
            Direction::Down => {
                if position.0 + 1 == grid.rows {
                    return None;
                }
                Some((position.0 + 1, position.1))
            }
            Direction::Left => {
                if position.1 == 0 {
                    return None;
                }
                Some((position.0, position.1 - 1))
            }
        }
    }
}

fn parse(input: &str) -> (Grid, Point) {
    let mut grid = Grid::new(input);
    let guard_position = grid.search('^').unwrap();
    grid.set(guard_position, '.');
    (grid, guard_position)
}

crate::aoctest!(41, 4647, 6, 1723);