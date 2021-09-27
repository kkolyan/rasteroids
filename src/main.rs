extern crate pancurses;

use std::collections::HashMap;
use std::fmt::Debug;
use std::io::BufRead;

use pancurses::Input;
use rand::Rng;

struct Object {
    obj_type: ObjectType,
    pos: Cell,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ObjectType {
    Enemy,
    Player,
    Item,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Cell {
    x: i8,
    y: i8,
}

struct Board {
    w: i8,
    h: i8,
    objects: Vec<Object>,
    scene_index: HashMap<Cell, ObjectType>,
}

impl Board {
    fn draw(&mut self, s: &mut String) {
        for obj in &self.objects {
            self.scene_index.insert(obj.pos, obj.obj_type);
        }
        for y in 0..self.h {
            for x in 0..self.w {
                s.push(match self.scene_index.get(&Cell { x, y }) {
                    None => { ' ' }
                    Some(value) => match value {
                        ObjectType::Enemy => { 'o' }
                        ObjectType::Player => { '^' }
                        ObjectType::Item => { '+' }
                    },
                });
            }
            s.push_str("\r\n");
        }
        self.scene_index.clear();
    }
}

fn main() {
    let mut board = Board { w: 8, h: 8, objects: Vec::new(), scene_index: HashMap::new() };

    board.objects.push(Object { pos: Cell { x: &board.w / 2, y: &board.h - 1 }, obj_type: ObjectType::Player });

    let mut buffer = String::new();

    let window = pancurses::initscr();
    window.keypad(true);

    let mut failed = false;

    loop {
        let player_pos = board.objects.iter().find(|it| true).map(|it| it.pos);
        for mut obj in &mut board.objects {
            if obj.obj_type == ObjectType::Enemy {
                obj.pos = Cell { x: obj.pos.x, y: obj.pos.y + 1 };
                if player_pos == Some(obj.pos) {
                    failed = true;
                }
            }
        }

        for x in 0..board.w {
            if rand::thread_rng().gen_ratio(1, 4) {
                board.objects.push(Object { obj_type: ObjectType::Enemy, pos: Cell { x, y: 0 } })
            }
        }
        buffer.clear();
        board.draw(&mut buffer);

        window.clear();
        window.addstr(&buffer);
        window.refresh();

        loop {
            match window.getch() {
                None => {}
                Some(c) => {
                    if !failed {
                        match c {
                            Input::KeyLeft => {
                                for mut obj in &mut board.objects {
                                    if obj.obj_type == ObjectType::Player && obj.pos.x > 1 {
                                        obj.pos = Cell { x: obj.pos.x - 1, y: obj.pos.y }
                                    }
                                }
                                break;
                            }
                            Input::KeyRight => {
                                for mut obj in &mut board.objects {
                                    if obj.obj_type == ObjectType::Player && obj.pos.x < board.w - 1 {
                                        obj.pos = Cell { x: obj.pos.x + 1, y: obj.pos.y }
                                    }
                                }
                                break;
                            }
                            Input::KeyUp => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
