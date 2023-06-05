use rand::Rng;
use ncurses;
use std::{thread, time::Duration};

const KEY_ONE : i32 = 49;
const KEY_TWO : i32 = 50;
const KEY_THREE : i32 = 51;
const KEY_FOUR  : i32 = 52;
const KEY_FIVE  : i32 = 53;
const KEY_SIX   : i32 = 54;
const KEY_SEVEN : i32 = 55;
const KEY_EIGHT : i32 = 56;
const KEY_NINE  : i32 = 57;

const KEY_LOWER_T : i32 = 116;
const KEY_UPPER_T : i32 = 84;

struct Pos {
	y : i32,
	x : i32

}

fn random_position(max_y: i32, max_x: i32) -> Pos 
{
	// https://rust-random.github.io/book/guide-start.html
	let mut rng = rand::thread_rng();
	Pos { y: rng.gen_range(0..max_y), 
		x: rng.gen_range(0..max_x) }
}

fn teleport(pos_y:i32, pos_x:i32, new_pos: &Pos) 
{
	// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
/*
	let dx:i32 = new_pos.x - pos_x;
	let dy:i32 = new_pos.y - pos_y;
	let mut big_d:i32 = 2 * dy  - dx;

	let mut y:i32 = pos_y;
	let mut x:i32 = pos_x;

	while x < new_pos.x {
		ncurses::mvaddch(y, x, 'X' as u32);
		if big_d > 0 {
			y += 1;
			big_d = big_d - 2*dx;
		}
		big_d = big_d + 2*dy;
		x += 1;
	}
*/
	if abs(new_pos.y - pos_y) < abs(new_pos.x - pos_x) {
		if pos_x > new_pos.x {
			plotLineLow(new_pos.x, new_pos.y, pos_y, pos_x);
		}
		else {
			plotLineLow(pos_x, pos_y, new_pos.x, new_pos.y );
		}
	}
	else {
		if pos_y > new_pos.y { 
			plotLineHigh( );
		}
		else { 
			plotLineHigh( );
		}
	}
}

fn main() 
{
	let win = ncurses::initscr();
	ncurses::raw();
	ncurses::keypad(win, true);
	ncurses::noecho();
	ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

	let mut max_x: i32 = 0;
	let mut max_y: i32 = 0;
	ncurses::getmaxyx(win, &mut max_y, &mut max_x);

	let mut pos_x:i32 = 10;
	let mut pos_y:i32 = 10;
	ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
	ncurses::refresh();

	loop {
		let ch = ncurses::getch();
		match ch {
			KEY_ONE => {
				if pos_y+1 < max_y && pos_x-1 >= 0 {
					ncurses::mvaddch(pos_y, pos_x, ' ' as u32);
					pos_y += 1;
					pos_x -= 1;
					ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
				}
			}
			KEY_TWO => {
				if pos_y+1 < max_y {
					ncurses::mvaddch(pos_y, pos_x, ' ' as u32);
					pos_y += 1;
					ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
				}
			}
			KEY_THREE => {
				if pos_y+1 < max_y && pos_x+1 < max_x {
					ncurses::mvaddch(pos_y, pos_x, ' ' as u32);
					pos_y += 1;
					pos_x += 1;
					ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
				}
			}
			KEY_FOUR => {
				if pos_x-1 >= 0 {
					ncurses::mvaddch(pos_y, pos_x, ' ' as u32);
					pos_x -= 1;
					ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
				}
			}
			KEY_FIVE => {
				// stay
			}
			KEY_SIX  => {
				if pos_x+1 < max_x {
					ncurses::mvaddch(pos_y, pos_x, ' ' as u32);
					pos_x += 1;
					ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
				}
			}
			KEY_SEVEN => {
				if pos_y-1 >=0 && pos_x-1 >= 0 {
					ncurses::mvaddch(pos_y, pos_x, ' ' as u32);
					pos_y -= 1;
					pos_x -= 1;
					ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
				}
			}
			KEY_EIGHT => {
				if pos_y-1 >=0 {
					ncurses::mvaddch(pos_y, pos_x, ' ' as u32);
					pos_y -= 1;
					ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
				}
			}
			KEY_NINE => {
				if pos_y-1 >=0 && pos_x+1 < max_x {
					ncurses::mvaddch(pos_y, pos_x, ' ' as u32);
					pos_y -= 1;
					pos_x += 1;
					ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
				}
			}
			KEY_UPPER_T | KEY_LOWER_T => {
				let new_pos = random_position(max_y, max_x);

				let s = format!("y={} x={}", new_pos.y, new_pos.x);
				ncurses::mvprintw(max_y-1, 0, &s);

				teleport(pos_y, pos_x, &new_pos);
//				ncurses::mvaddch(pos_y, pos_x, ' ' as u32);
				pos_y = new_pos.y;
				pos_x = new_pos.x;
//				ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
			}
			ncurses::KEY_F1 => {
				ncurses::mvprintw(max_y/2, 10, "Hello F1 !!!");
				break;
			}
			ncurses::KEY_F2 => {
				let s = format!("y={} x={}", pos_y, pos_x);
				ncurses::mvprintw(max_y-1, 0, &s);
				for _ in 1..4 {
					ncurses::mvaddch(pos_y, pos_x, 'x' as u32);
					ncurses::refresh();
					thread::sleep(Duration::from_millis(1000));
					ncurses::mvaddch(pos_y, pos_x, 'X' as u32);
					ncurses::refresh();
					thread::sleep(Duration::from_millis(1000));
				}
			}
			_ => {}
		}
		ncurses::refresh();
	}

//	ncurses::getch();

	ncurses::endwin();

	println!("x={max_x} y={max_y}");
}
