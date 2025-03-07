use rand::prelude::*;
use std::{borrow::{Borrow, BorrowMut}, ffi::CString, fmt::{self, Display}};
const GRID_WIDTH: u32 = 20;
const GRID_HEIGHT: u32 = 15;
const GRID_UNIT_SIZE: u8 = 40;
const SEGMENT_SIZE: u8 = 30;
const APPLE_SIZE: u8 = 30;
const FRAME_DELTA_TIME_MILIS: u32 = 125;
const GRID_SEGMENTS_COUNT: u32 = GRID_HEIGHT * GRID_WIDTH;  

static mut apple_x: u32 = 10;
static mut apple_y: u32 = 7;
static mut score: u32 = 0;

#[repr(C)]
#[derive(Clone, Copy)] 
enum SegmentDirection {
    UP = 0,
    DOWN = 1,
    LEFT = 2,
    RIGHT = 3
}


struct SnakeSegment {
    direction: SegmentDirection,
    x: u32,
    y: u32
}



extern "C" {
    fn init(grid_width: i32, grid_height: i32, grid_unit_size: u8, segment_size: u8, apple_size: u8);
    fn get_time_milis() -> u32;
    fn clear_screen();
    fn draw_segment(segment_x: u32, segment_y: u32, dir: SegmentDirection);
    fn draw_apple(apple_x: u32, apple_y: u32);
    fn render();
    fn update_SDL();
    fn finish_game();
    fn set_title(title: *const u8);
    static mut up: bool;
    static mut down: bool;
    static mut left: bool;
    static mut right: bool;
    static mut game_over: bool;

} 

pub struct LinkedListNode<T> {
    value: T,
    next: Option<Box<LinkedListNode<T>>>
}

impl<T> LinkedListNode<T> {
    pub fn new(value: T) -> LinkedListNode<T>{
        LinkedListNode {
            value: value,
            next: None
        }
    }

    pub fn delete_end(self: &mut LinkedListNode<T>) -> bool {
        let mut cur: &mut LinkedListNode<T> = self;
        if !cur.has_next() { // check if next exists
            return false
        }
        loop {
            if !cur.next.as_ref().unwrap().has_next() { // check if next next exists
                cur.next = None;
                break;
            }
            cur = cur.next.as_mut().unwrap().borrow_mut(); // borrow next which was already checked to exist
        }
        true
    }

    pub fn has_next(self: &LinkedListNode<T>) -> bool {
        match self.next {
            None => {false},
            Some(_) => {true}
        }
    }
}

impl<T: Display> fmt::Display for LinkedListNode<T> {
    fn fmt(self: &LinkedListNode<T>, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from("");
        let mut cur: &LinkedListNode<T> = self;
        loop {
            s += &format!("->{}", cur.value);
            match cur.next {
                None => break,
                Some(ref next) => cur = next.borrow(),
            }
        }
        write!(f, "{}", s)
    } 
}

fn find_new_direction(mut cur_dir: SegmentDirection) -> SegmentDirection {
    unsafe {
        match cur_dir {
            SegmentDirection::UP => {down = false},
            SegmentDirection::DOWN => {up = false},
            SegmentDirection::LEFT => {right = false},
            SegmentDirection::RIGHT => {left = false}
        }
        if up {
            cur_dir = SegmentDirection::UP;
        }
        else if down {
            cur_dir = SegmentDirection::DOWN;
        }    
        else if left {
            cur_dir = SegmentDirection::LEFT;
        }
        else if right {
            cur_dir = SegmentDirection::RIGHT;
        }
    }
    cur_dir
}

fn is_pos_in_snake(mut cur: &Box<LinkedListNode<SnakeSegment>>, x: u32, y: u32) -> bool {
    loop {
        if cur.value.x == x && cur.value.y == y {
            return true;
        }
        match cur.next {
            None => {return false;},
            Some(ref next_ref) => {cur = next_ref}
        }
    }
}

fn spawn_apple(head: &Box<LinkedListNode<SnakeSegment>>) {
    // TODO: switch to a normal algorithm without a potentially infinite runtime
    let mut rng = rand::rng();
    loop {
        let mut x: u32 = rng.random();
        x = x % GRID_WIDTH;
        let mut y: u32 = rng.random();
        y = y % GRID_HEIGHT;
        if !is_pos_in_snake(&head, x, y) {
            unsafe {
                apple_x = x;
                apple_y = y;
            }
            break;
        }
    }

}

fn handle_game_logic(head: Box<LinkedListNode<SnakeSegment>>, new_dir: SegmentDirection) -> Box<LinkedListNode<SnakeSegment>> {
    let mut x: u32 = head.value.x;
    let mut y: u32 = head.value.y;
    unsafe {
        match new_dir{
            SegmentDirection::UP => {if y > 0 { y -= 1;} else {game_over = true}},
            SegmentDirection::DOWN => {if y < GRID_HEIGHT - 1{y += 1;} else {game_over = true}} ,
            SegmentDirection::LEFT => {if x > 0 {x -= 1} else {game_over = true}},
            SegmentDirection::RIGHT => {if x < GRID_WIDTH - 1 {x += 1} else {game_over = true}},
            
        }
    }
    let mut new_head = Box::new(LinkedListNode{ value: SnakeSegment{direction: new_dir, x: x, y: y}, next: Some(head) });
    if !unsafe {x == apple_x && y == apple_y} {
        new_head.delete_end();
        match new_head.next {
            None => {},
            Some(ref next_ref) => {
                let cur = next_ref;
                let collision = is_pos_in_snake(cur, x, y);
                unsafe {game_over = collision};
            }
        }
    } else {
        spawn_apple(&new_head);
        unsafe { score += 1; };
    }
    
    new_head
}

fn main() {
    let mut head: Box<LinkedListNode<SnakeSegment>> = Box::from(LinkedListNode{value: SnakeSegment{direction: SegmentDirection::RIGHT, x: 0, y: 0}, next: None });
    let mut last_time: u32;
    let mut next_dir = head.value.direction; 

    unsafe {
        init(GRID_WIDTH as i32, GRID_HEIGHT as i32, GRID_UNIT_SIZE, SEGMENT_SIZE, APPLE_SIZE);
        last_time = get_time_milis();
    }
    loop {
        
        let cur_time: u32;
        unsafe {
            update_SDL();
            cur_time = get_time_milis();
        }
        next_dir = find_new_direction(next_dir);

        
        if (cur_time - last_time) >= FRAME_DELTA_TIME_MILIS {
            last_time = cur_time;
            head = handle_game_logic(head, next_dir);
            let mut cur = &head;
            unsafe {
                set_title(CString::new(format!("score:  {score}").as_str()).unwrap().as_bytes_with_nul().as_ptr());
                clear_screen();
            }
            loop {
                unsafe{
                    draw_segment(cur.value.x, cur.value.y, cur.value.direction);
                }
                match cur.next {
                    None => {break;},
                    Some(ref next) => {cur = next}
                }
            }
            unsafe {
                draw_apple(apple_x, apple_y);
                render();
            }
        }
        unsafe {
            if game_over {
                finish_game();
                break;
            }
        }
    }

}
