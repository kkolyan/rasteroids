extern crate pancurses;

use std::collections::HashMap;
use std::ops;

use pancurses::{Input, Window};
use rand::{Rng};

fn main() {
    println!("{:o}", 42);
    let mut world = World { w: 8, h: 8, player: None, enemies: Vec::new() };
    let mut board = BoardRenderer { scene_index: HashMap::new() };

    world.restart_game();

    let mut screen_buffer = String::new();

    let terminal: Window = pancurses::initscr();
    terminal.keypad(true);

    world.return_one().as_mut();

    loop {
        screen_buffer.clear();
        board.draw(&world, &mut screen_buffer);

        terminal.clear();
        terminal.addstr(&screen_buffer);
        terminal.refresh();

        world.update(&terminal);
    }
}

// model

#[derive(Copy, Clone, Eq, PartialEq)]
enum ObjectType {
    Enemy,
    Player,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Vec2i {
    x: i8,
    y: i8,
}

struct Enemy {
    pos: Vec2i,
}

struct World {
    w: i8,
    h: i8,
    player: Option<Vec2i>,
    enemies: Vec<Enemy>,
}

enum GameInput {
    None,
    Move { dir: Vec2i },
    Restart,
}

struct BoardRenderer {
    scene_index: HashMap<Vec2i, ObjectType>,
}

trait InputProvider {
    fn poll_input(&self) -> GameInput;
}

// game logic

impl World {

    fn return_one(&self) -> Result<i32, &str> {
        return Ok(42);
    }

    fn restart_game(&mut self) {
        self.player = Some(Vec2i { x: self.w / 2, y: self.h - 1 });
        self.enemies.clear()
    }

    fn update(&mut self, window: &impl InputProvider) {
        match window.poll_input() {
            GameInput::Move { dir } => {
                if let Some(player) = self.player {
                    let new_pos = player + dir;
                    if new_pos.x >= 0 && new_pos.x < self.w {
                        self.player = Some(new_pos)
                    }
                }
            }
            GameInput::None => {}
            GameInput::Restart => {
                if self.player.is_none() {
                    self.restart_game();
                    return;
                }
            }
        }

        for x in &mut self.enemies {
            x.pos += Vec2i { x: 0, y: 1 };
            if Some(x.pos) == self.player {
                self.player = None
            }
        }

        for x in 0..self.w {
            if rand::thread_rng().gen_ratio(1, 4) {
                self.enemies.push(Enemy { pos: Vec2i { x, y: 0 } });
            }
        }
    }
}

impl InputProvider for Window {
    fn poll_input(&self) -> GameInput {
        loop {
            if let Some(c) = self.getch() {
                match c {
                    Input::Character('\u{1b}') => return GameInput::Restart,//ESC
                    Input::KeyLeft => return GameInput::Move { dir: Vec2i { x: -1, y: 0 } },
                    Input::KeyRight => return GameInput::Move { dir: Vec2i { x: 1, y: 0 } },
                    Input::KeyUp => return GameInput::None,
                    _ => {},
                }
            }
        }
    }
}

impl BoardRenderer {
    fn draw(&mut self, world: &World, s: &mut String) {
        self.scene_index.clear();

        if let Some(player_pos) = &world.player {
            self.scene_index.insert(*player_pos, ObjectType::Player);
        }

        for enemy_pos in &world.enemies {
            self.scene_index.insert(enemy_pos.pos, ObjectType::Enemy);
        }
        for y in 0..world.h {
            for x in 0..world.w {
                s.push(match self.scene_index.get(&Vec2i { x, y }) {
                    None => ' ',
                    Some(ObjectType::Enemy)=> 'o',
                    Some(ObjectType::Player) => '^',
                });
            }
            s.push_str("\n");
        }
        if world.player.is_none() {
            s.push_str("GAMEOVER\n");
            s.push_str("PressESC\n");
        }
    }
}

impl ops::Add for Vec2i {
    type Output = Vec2i;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        return Vec2i { x: self.x + rhs.x, y: self.y + rhs.y };
    }
}

impl ops::AddAssign for Vec2i {

    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
