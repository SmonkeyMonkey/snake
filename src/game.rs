use std::collections::VecDeque;

use crate::random::random_range;

pub type Position = (usize, usize);
#[derive(Clone,Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>, // head - first item, tail - last item
    pub direction: Direction,
    next_direction: Direction,
    pub food: Position,
    pub lost: bool,
    pub score: usize
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width - 3).max(0), height / 2)].into_iter().collect(),
            direction: Direction::Left,
            next_direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
            lost: false,
            score: 0,
        }
    }
    pub fn change_direction(&mut self, direction: Direction) {
        if self.lost {
            return;
        }
        match (&self.direction, direction) {
            (Direction::Up, Direction::Up)
            | (Direction::Up, Direction::Down)
            | (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Down, Direction::Up)
            | (Direction::Down, Direction::Down)
            | (Direction::Left, Direction::Right)
            | (Direction::Left, Direction::Left) => {}
            (_, direcection) => self.next_direction = direcection,
        }
    }

    pub fn tick(&mut self) {
        if self.lost && self.snake.len() == 0 {
            return;
        }
        self.direction = self.next_direction;
        // move snake
        // remove the last tail and push a new head

        let (x, y) = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.lost = true
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                let free_positions = (0..self.height)
                    .flat_map(|y| (0..self.height).map(move |x| (x, y)))
                    .filter(|pos| !self.snake.contains(pos))
                    .collect::<Vec<_>>();
                self.score += 1;

                if free_positions.is_empty() {
                    self.lost = true;
                    return;
                }
                self.food = free_positions[random_range(0, free_positions.len())];
            }    
            self.snake.push_front(new_head);
            
        }
    }
    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }
    
}
