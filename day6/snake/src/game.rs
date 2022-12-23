use cursive::{Vec2, XY };
use std::collections::LinkedList;

use rand::{thread_rng, Rng, rngs::ThreadRng};
// use std::cmp::max;

pub struct Game {
    pub size: Vec2,
    pub snake: LinkedList<XY<usize>>,
    pub apple: XY<usize>,
    pub rng: ThreadRng,
}

#[derive(Clone, Copy)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Game {
    pub fn new() -> Self {
        let game = Game {
            size: Vec2 { x: 20, y: 20 },
            snake: LinkedList::from([ XY::new(10, 10)]),
            apple: XY::new(5, 5),
            rng: thread_rng(),
        };

        return game;
      }

    pub fn step(&mut self, dir: Direction) -> bool {
      let head = self.snake.front().unwrap();

      let next_head = match dir {
        Direction::Up => XY::new(head.x, head.y - 1),
        Direction::Down => XY::new(head.x, head.y + 1),
        Direction::Left => XY::new(head.x - 1, head.y),
        Direction::Right => XY::new(head.x + 1, head.y),
      };

      if (next_head.x > self.size.x) || (next_head.y > self.size.y) {
        return false;
      }

      self.snake.push_front(next_head);

      if (next_head.x != self.apple.x) || (next_head.y != self.apple.y) {
        self.snake.pop_back();
      } else {
        // ate an apple
        self.apple.x = self.rng.gen_range(0..self.size.x);
        self.apple.y = self.rng.gen_range(0..self.size.y);
      }
      
      return true;
    }
}

#[cfg(test)]
mod tests {
  use super::*;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[test]
    fn test_can_move() {
      let mut game = Game::new();
      game.step(Direction::Up);
      game.step(Direction::Up);
      game.step(Direction::Up);

      assert_eq!(game.snake.front().unwrap().x, 10);
      assert_eq!(game.snake.front().unwrap().y, 7);
      assert_eq!(game.snake.len(), 1);
    }

    #[test]
    fn test_can_eat_apple() {
      let mut game = Game::new();
      game.step(Direction::Up);
      game.step(Direction::Up);
      game.step(Direction::Up);
      game.step(Direction::Up);
      game.step(Direction::Up);
      game.step(Direction::Left);
      game.step(Direction::Left);
      game.step(Direction::Left);
      game.step(Direction::Left);
      game.step(Direction::Left);


      assert_eq!(game.snake.front().unwrap().x, 5);
      assert_eq!(game.snake.front().unwrap().y, 5);
      assert_eq!(game.snake.len(), 2);
    }

    #[test]    
    fn test_can_hit_a_wall() {
      let mut game = Game::new();
      let mut steps = 0;
      for i in 0..100 {
        if !game.step(Direction::Up) {
          break;
        }
        steps += 1;
      }

      assert_eq!(steps, 9);
    }

}