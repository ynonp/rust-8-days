use cursive::{
    event::{Event, EventResult, Key},    
    view::{Nameable},    
    Printer, Vec2,
};
use game::{Direction};
use std::{thread};
use std::time::Duration;
mod game;

struct GameView {
    // Actual board, unknown to the player.
    board: game::Game,
    direction: Direction,
}

impl GameView {
    pub fn new() -> Self {
        return GameView {
            board: game::Game::new(),
            direction: Direction::Down,
        };
    }

    pub fn step(&mut self) -> bool {
        return self.board.step(self.direction);
    }
}

impl cursive::view::View for GameView {
    fn draw(&self, printer: &Printer) {
        printer.print(self.board.snake.front().unwrap(), match self.direction {
            Direction::Up => "^",
            Direction::Down => "V",
            Direction::Left => "<",
            Direction::Right => ">",
        });
        for snake_pos in self.board.snake.iter().skip(1) {
            printer.print(snake_pos, "X");
        }        

        printer.print(self.board.apple, "O");
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(key) => {
                self.direction = match key {
                    Key::Down => Direction::Down,
                    Key::Up => Direction::Up,
                    Key::Left => Direction::Left,
                    Key::Right => Direction::Right,
                    _ => self.direction,
                };
                self.step();
            }
            _ => (),
        }
        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.board.size.map_x(|x| 2 * x)
    }
}


fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    let game = GameView::new();
    let tx = siv.cb_sink().clone();

    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(500));
            game.step();
        }        
    });

    thread::spawn(move || {
        // game.board.step(Direction::Up);
        loop {
            thread::sleep(Duration::from_millis(500));
            tx.send(Box::new(|si| {
                si.call_on_name("game", |view: &mut GameView|  {
                    let still_alive = view.step();
                    if !still_alive {
                        panic!("You lose");
                    }
                }).unwrap();
            })).unwrap();
        }
    });

    siv.add_layer(game.with_name("game"));
    
    siv.set_fps(24);
    // Starts the event loop.
    siv.run();
}