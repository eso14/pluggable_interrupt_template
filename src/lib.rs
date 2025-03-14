#![no_std]

use num::Integer;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::vga_buffer::{
    is_drawable, plot, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH,
};

use core::{
    clone::Clone,
    cmp::{min, Eq, PartialEq},
    iter::Iterator,
    marker::Copy,
    prelude::rust_2024::derive,
};

const SNAKE_CHAR: char = 'O';
const FOOD_CHAR: char = 'X';
const EMPTY_CHAR: char = ' ';

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SnakeGame {
    body_x: [usize; BUFFER_WIDTH * BUFFER_HEIGHT], 
    body_y: [usize; BUFFER_WIDTH * BUFFER_HEIGHT],
    length: usize,
    head_index: usize,
    food_x: usize,
    food_y: usize,
    dx: isize,
    dy: isize,
    game_over: bool,
}





impl Default for SnakeGame {
    fn default() -> Self {
        let start_x = BUFFER_WIDTH / 2;
        let start_y = BUFFER_HEIGHT / 2;
        let mut body_x = [0; BUFFER_WIDTH * BUFFER_HEIGHT];
        let mut body_y = [0; BUFFER_WIDTH * BUFFER_HEIGHT];

        body_x[0] = start_x;
        body_y[0] = start_y;

        Self {
            body_x,
            body_y,
            length: 3,
            head_index: 0,
            food_x: BUFFER_WIDTH / 4,
            food_y: BUFFER_HEIGHT / 4,
            dx: 1,
            dy: 0,
            game_over: false,
        }
    }
}

impl SnakeGame {
    pub fn new() -> Self {
        let start_x = BUFFER_WIDTH / 2;
        let start_y = BUFFER_HEIGHT / 2;
        let mut body_x = [0; BUFFER_WIDTH * BUFFER_HEIGHT];
        let mut body_y = [0; BUFFER_WIDTH * BUFFER_HEIGHT];

        body_x[0] = start_x;
        body_y[0] = start_y;

        Self {
            body_x,
            body_y,
            length: 3,
            head_index: 0,
            food_x: BUFFER_WIDTH / 4,
            food_y: BUFFER_HEIGHT / 4,
            dx: 1,
            dy: 0,
            game_over: false,
        }
    }

    pub fn tick(&mut self) {
        if self.game_over {
            self.display_game_over();
            return;
        }

        self.clear_snake();
        self.update_snake();
        self.check_collisions();
        self.draw_snake();
        self.draw_food();
        self.display_score();
    }

    fn clear_snake(&self) {
        for i in 0..self.length {
            plot(
                EMPTY_CHAR,
                self.body_x[i],
                self.body_y[i],
                ColorCode::new(Color::Black, Color::Black),
            );
        }
    }

    fn update_snake(&mut self) {
        let new_x = self.safe_add(self.body_x[self.head_index], self.dx, BUFFER_WIDTH);
        let new_y = self.safe_add(self.body_y[self.head_index], self.dy, BUFFER_HEIGHT);

        self.head_index = (self.head_index + 1) % self.length;
        self.body_x[self.head_index] = new_x;
        self.body_y[self.head_index] = new_y;

        if new_x == self.food_x && new_y == self.food_y {
            self.length = min(self.length + 1, BUFFER_WIDTH * BUFFER_HEIGHT);
            self.spawn_food();
        }
    }

    fn check_collisions(&mut self) {
        let head_x = self.body_x[self.head_index];
        let head_y = self.body_y[self.head_index];

        for i in 0..self.length {
            if i != self.head_index && self.body_x[i] == head_x && self.body_y[i] == head_y {
                self.game_over = true;
            }
        }
    }

    fn draw_snake(&self) {
        for i in 0..self.length {
            plot(
                SNAKE_CHAR,
                self.body_x[i],
                self.body_y[i],
                ColorCode::new(Color::Green, Color::Black),
            );
        }
    }

    fn draw_food(&self) {
        plot(
            FOOD_CHAR,
            self.food_x,
            self.food_y,
            ColorCode::new(Color::Red, Color::Black),
        );
    }

    fn spawn_food(&mut self) {
        self.food_x = (self.food_x + 7) % BUFFER_WIDTH;
        self.food_y = (self.food_y + 5) % BUFFER_HEIGHT;
    }

    fn safe_add(&self, a: usize, b: isize, limit: usize) -> usize {
        let sum = a as isize + b;
        if sum < 0 {
            (sum + limit as isize) as usize
        } else {
            (sum % limit as isize) as usize
        }
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c),
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        if self.game_over {
            return;
        }
        
        match key {
            KeyCode::ArrowLeft if self.dx == 0 => {
                self.dx = -1;
                self.dy = 0;
            }
            KeyCode::ArrowRight if self.dx == 0 => {
                self.dx = 1;
                self.dy = 0;
            }
            KeyCode::ArrowUp if self.dy == 0 => {
                self.dx = 0;
                self.dy = -1;
            }
            KeyCode::ArrowDown if self.dy == 0 => {
                self.dx = 0;
                self.dy = 1;
            }
            _ => {}
        }
    }
    fn clear_screen(&self) {
        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                plot(EMPTY_CHAR, x, y, ColorCode::new(Color::Black, Color::Black));
            }
        }
    }


    fn handle_unicode(&mut self, key: char) {
        if self.game_over {
            if key == 'r' {
                self.clear_screen();
                *self = SnakeGame::new(); // Restart game
            }
        } else if key == 'q' {
            self.game_over = true; // Quit game
        }
    }

    fn display_score(&self) {
        let score = self.length - 3; // Initial length is 3
        let score_chars = [
            ('S', 0), ('c', 1), ('o', 2), ('r', 3), ('e', 4), (':', 5), (' ', 6),
            (((score / 10) as u8 + b'0') as char, 7),
            (((score % 10) as u8 + b'0') as char, 8),
        ];
        for (c, i) in score_chars.iter() {
            plot(*c, *i, BUFFER_HEIGHT - 1, ColorCode::new(Color::White, Color::Black));
        }
    }

    fn display_game_over(&self) {
        let message = [
            ('G', 0), ('A', 1), ('M', 2), ('E', 3), (' ', 4), ('O', 5), ('V', 6), ('E', 7), ('R', 8), ('!', 9),
            (' ', 10), ('P', 11), ('r', 12), ('e', 13), ('s', 14), ('s', 15), (' ', 16), ('r', 17), (' ', 18),
            ('t', 19), ('o', 20), (' ', 21), ('r', 22), ('e', 23), ('s', 24), ('t', 25), ('a', 26), ('r', 27), ('t', 28), ('.', 29)
        ];
        for (c, i) in message.iter() {
            plot(*c, BUFFER_WIDTH / 2 - 15 + *i, BUFFER_HEIGHT / 2, ColorCode::new(Color::Red, Color::Black));
        }
    }
}

