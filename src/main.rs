use std::{borrow::{Borrow, BorrowMut}, fmt::{self, Display}};
const GRID_WIDTH: u32 = 20;
const GRID_HEIGHT: u32 = 15;
const GRID_UNIT_SIZE: u8 = 40;
const SEGMENT_SIZE: u8 = 30;
const APPLE_SIZE: u8 = 30;
// static mut field: [[bool; GRID_WIDTH as usize]; GRID_HEIGHT as usize] = [[false; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
static mut apple_x: u32 = 10;
static mut apple_y: u32 = 7;

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

fn find_new_direction(head: &Box<LinkedListNode<SnakeSegment>>) -> SegmentDirection {
    let mut cur_dir: SegmentDirection = head.value.direction;
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
    let mut new_head = LinkedListNode{ value: SnakeSegment{direction: new_dir, x: x, y: y}, next: Some(head) };
    new_head.delete_end();
    Box::new(new_head)
}

fn main() {
    let mut head: Box<LinkedListNode<SnakeSegment>> = Box::from(LinkedListNode{value: SnakeSegment{direction: SegmentDirection::LEFT, x: 5, y: 5}, next: None });
    // println!("Hello, world!");
    unsafe {
        init(GRID_WIDTH as i32, GRID_HEIGHT as i32, GRID_UNIT_SIZE, SEGMENT_SIZE, APPLE_SIZE);
        set_title("test".as_ptr());
    }
    loop {
        let new_dir = find_new_direction(&head);

        
        unsafe {
            update_SDL();
            clear_screen();
        }
        head = handle_game_logic(head, new_dir);
        let mut cur = &head;
        loop {
            unsafe{
                draw_segment(cur.value.x, cur.value.y, cur.value.direction);
            }
            match cur.next {
                None => {break;},
                Some(ref next) => {cur = next.borrow()}
            }
        }
        unsafe {
            draw_apple(apple_x, apple_y);
            render();
        }
        if false {
            break;
        }
    }

}
