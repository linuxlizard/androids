use ncurses;

const KEY_ONE : i32 = 49;
const KEY_TWO : i32 = 50;
const KEY_THREE : i32 = 51;
const KEY_FOUR  : i32 = 52;
const KEY_FIVE  : i32 = 53;
const KEY_SIX   : i32 = 54;
const KEY_SEVEN : i32 = 55;
const KEY_EIGHT : i32 = 56;
const KEY_NINE  : i32 = 57;

fn main() {
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
			ncurses::KEY_F1 => {
				ncurses::mvprintw(max_y/2, 10, "Hello F1 !!!");
				break;
			}
			_ => {}
		}
		ncurses::refresh();
	}

//	ncurses::getch();

	ncurses::endwin();

	println!("x={max_x} y={max_y}");
}
