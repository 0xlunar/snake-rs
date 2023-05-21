use rand::Rng;
use clearscreen;
use std::{thread, time::Duration};
use crossterm::{
    event::{read, KeyEvent, Event, KeyCode},
};

fn main(){
    loop {
        let mut board = Board::new(24, 24);
        board.start();
    
        println!("Game over! restarting in 15 seconds.");
        thread::sleep(Duration::from_millis(15000));

    }
}

type Node<T> = Option<Box<T>>;

#[derive(Debug, Clone, Copy)]
enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug)]
struct Board {
    height: i32,
    width: i32,
    fruit_pos: (i32, i32),
    score: i32,
    alive: bool,
    direction: Directions,
    snake: Snake,
}

#[derive(Debug, Clone)]
struct Snake {
    head: (i32, i32),
    tail: Node<Snake>,
}

impl Snake {
    fn new(x: i32, y: i32) -> Snake {
        Snake { head: (x, y), tail: None }
    }

    pub fn eat_fruit(&mut self) {
        match &mut self.tail {
            None => self.tail = Some(Box::new(Snake::new(self.head.0, self.head.1))),
            Some(snake) => snake.eat_fruit(),
        }
    }

    pub fn move_direction(&mut self, direction: Directions) {
        self.move_body();
        match direction {
            Directions::UP => self.head.0 -= 1,
            Directions::DOWN => self.head.0 += 1,
            Directions::LEFT => self.head.1 -= 1,
            Directions::RIGHT => self.head.1 += 1
        }
    }

    fn move_body(&mut self) {
        match &mut self.tail {
            Some(snake) => {
                snake.move_body();
                snake.head.0 = self.head.0;
                snake.head.1 = self.head.1; 
            },
            None => (),
        }
    }

    fn did_self_collide(&self, head_x: i32, head_y: i32) -> bool {
        match &self.tail {
            Some(snake) => {
                 if snake.head.0 == head_x && snake.head.1 == head_y {
                    return true;
                 } else {
                    return snake.did_self_collide(head_x, head_y);
                 }
            },
            None => return false,
        }
    }

    fn render(&self, grid: &mut[&mut[char]]) {
        grid[self.head.0 as usize][self.head.1 as usize] = '$';
        match &self.tail {
            Some(snake) => snake.render_body(grid),
            None => (),
        }
    }

    fn render_body(&self, grid: &mut[&mut[char]]) {
        grid[self.head.0 as usize][self.head.1 as usize] = '#';
        match &self.tail {
            Some(snake) => snake.render_body(grid),
            None => (),
        }
    }
}

impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        let mut rng = rand::thread_rng();
        let rand_x: i32 = (rng.gen::<f32>() * width as f32) as i32;
        let rand_y: i32 = (rng.gen::<f32>() * height as f32) as i32;

        Board { height, width, score: 0, fruit_pos: (rand_x, rand_y), alive: true, direction: Directions::RIGHT, snake: Snake::new(height / 2, width / 2) }
    }

    pub fn move_snake(&mut self, direction: Directions) {
        if !self.alive {
            return;
        }
    
        self.snake.move_direction(direction);

        if self.snake.did_self_collide(self.snake.head.0, self.snake.head.1) {
            std::process::exit(0);
        }

        let head = &self.snake.head;
        if head.0 < 0 || head.0 >= self.width || head.1 < 0 || head.1 >= self.height {
            self.alive = false;
        } else if head.0 == self.fruit_pos.0 && head.1 == self.fruit_pos.1 {
            self.snake.eat_fruit();
            self.spawn_fruit();
            self.score += 1;
        }
    }

    fn spawn_fruit(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_x: i32 = (rng.gen::<f32>() * self.width as f32) as i32;
        let rand_y: i32 = (rng.gen::<f32>() * self.height as f32) as i32;

        self.fruit_pos = (rand_x, rand_y);
    }

    fn render(&self) {
        clearscreen::clear().expect("failed to clear");
        let mut grid = vec!['*'; (self.width * self.height).try_into().unwrap()];
        let mut grid: Vec<_> = grid.as_mut_slice().chunks_mut((self.width).try_into().unwrap()).collect();
        let grid = grid.as_mut_slice();

        if self.alive {
            self.snake.render(grid);
        }

        grid[self.fruit_pos.0 as usize][self.fruit_pos.1 as usize] = '@';
        
        let mut builder = String::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                builder.push(grid[i][j]);
            }
            builder.push('\n');
        }
        println!("X: {}, Y: {}, Score: {}", self.snake.head.0, self.snake.head.1, self.score);
        println!("{}", builder);
        println!("ESC = Quit | LEFT, RIGHT, UP, DOWN");
    }

    pub fn detect_input(&mut self) {
        let event = read();
        match event {
            Ok(e) => match e {
                Event::Key(KeyEvent { code, ..}) => {
                    match code {
                        KeyCode::Up => {
                            println!("UP");
                            self.direction = Directions::UP
                        },
                        KeyCode::Down => {
                            println!("DOWN");
                            self.direction = Directions::DOWN
                        },
                        KeyCode::Left =>  {
                            println!("LEFT");
                            self.direction = Directions::LEFT
                        },
                        KeyCode::Right =>  {
                            println!("RIGHT");
                            self.direction = Directions::RIGHT
                        },
                        KeyCode::Esc => std::process::exit(0),
                        _ => ()
                    }
                },
                _ => ()
            },
            _ => ()
        }
    }

    pub fn start(&mut self) {

        let fps = 1000 / 60;

        while self.alive {
            self.detect_input();
            self.move_snake(self.direction);
            self.render();
            thread::sleep(Duration::from_millis(fps));
        }
    }

}


