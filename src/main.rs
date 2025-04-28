/* davep 20250325 ; convert from ncurses to crossterm */

use rand::Rng;
use std::{thread, time::Duration};
//use crate::line;
use std::io::{self, Write};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, Print},
    event::{Event,KeyEvent,KeyCode,read}
};


struct Point {
    x: u16,
    y: u16,
}

// via ChatGPT4  (!!!)
fn teleport(stdout: &mut io::Stdout, p1: Point, p2: &Point) -> Vec<Point> 
{
    let mut points = Vec::new();
    let mut x1 = p1.x as i16;
    let mut y1 = p1.y as i16;
    let x2 = p2.x as i16;
    let y2 = p2.y as i16;

    let dx:i16 = if x2 > x1 { x2 - x1 } else { x1 - x2 };
//    let dx = (x2 - x1).abs();
    let dy:i16 = if y2 > y1 { y2 - y1 } else { y1 - y2 };
//    let dy = (y2 - y1).abs();

    let sx : i16;
    let sy : i16;

    if x1 < x2 {
        sx = 1;
    } else {
        sx = -1;
    }

    if y1 < y2 {
        sy = 1;
    } else {
        sy = -1;
    }

    let mut err:i16 = dx - dy;

    loop {
        points.push(Point { x: x1 as u16, y: y1 as u16 });

        crossterm::execute!(stdout, 
                cursor::MoveTo(x1 as u16, y1 as u16),
                Print("X")
                );

        if x1 == x2 && y1 == y2 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x1 += sx;
        }

        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }

    points
}

fn step_enemy(android: &mut Point, player: &Point) {
    // using the algorithm from Robots in BSDGames
    // https://github.com/vattam/BSDGames.git
    if android.y > player.y {
        android.y -= 1;
    } else if android.y < player.y {
        android.y += 1;
    }
    if android.x > player.x {
        android.x -= 1;
    } else if android.x < player.x {
        android.x += 1;
    }
}

fn random_position(max_y: u16, max_x: u16) -> Point {
    // https://rust-random.github.io/book/guide-start.html
    let mut rng = rand::thread_rng();
    Point {
        y: rng.gen_range(0..max_y),
        x: rng.gen_range(0..max_x),
    }
}

fn erase(stdout: &mut io::Stdout, mut points: Vec<Point>) {
    // don't erase current position
    points.pop();
    for p in points {
        thread::sleep(Duration::from_millis(5));
        crossterm::execute!(stdout, 
                cursor::MoveTo(p.x, p.y),
                Print(" ")
                );
    }
}

//fn teleport(pos_y:i32, pos_x:i32, new_pos: &Point)
//{
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
/*
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
*/
//}

fn move_androids(stdout: &mut io::Stdout, androids: &mut Vec<Point>, player: &Point)
{
    for a in androids.iter_mut() {
        stdout.queue(
            cursor::MoveTo(a.x, a.y)).unwrap()
            .queue(Print( " ")).unwrap();

        step_enemy(a, player);

        stdout.queue(
            cursor::MoveTo(a.x, a.y)).unwrap()
            .queue(Print( "A")).unwrap();

    }

}

fn main() -> io::Result<()> 
{
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    // top left is (1,1)
    let (max_x, max_y) = terminal::size().unwrap();

    let mut pos = random_position(max_y, max_x);
    stdout.queue(
       cursor::MoveTo(pos.x, pos.y))?
        .queue(style::Print("X"))?;


    let mut androids: Vec<Point> = Vec::new();
    while androids.len() < 5 {
        let p = random_position(max_y, max_x);
        if !(p.x == pos.x && p.y == pos.y) {
            stdout.queue(
               cursor::MoveTo(p.x, p.y))?
                .queue(style::Print("A"))?;

            androids.push(p);
        }
    }

    stdout.flush()?;

    loop {

        if let Event::Key(KeyEvent { code, .. }) = read()? {
            stdout.queue(
                cursor::MoveTo(pos.x, pos.y))?
                .queue(Print( " "))?;

            match code {
                KeyCode::Char('q') => { break; }
                KeyCode::Char('t') => {
                    let new_pos = random_position(max_y, max_x);
                    let tpoints = teleport(&mut stdout, Point { y: pos.y, x: pos.x }, &new_pos);
                    erase(&mut stdout, tpoints);
                    pos.y = new_pos.y;
                    pos.x = new_pos.x;
                }
                KeyCode::Char('1') => {
                    if pos.y + 1 < max_y && pos.x > 0 {
                        pos.y += 1;
                        pos.x -= 1;
                    }
                }
                KeyCode::Char('2') => {
                    if pos.y + 1 < max_y {
                        pos.y += 1;
                    }
                }
                KeyCode::Char('3') => {
                    if pos.y + 1 < max_y && pos.x + 1 < max_x {
                        pos.y += 1;
                        pos.x += 1;
                    }
                }
                KeyCode::Char('4') => {
                    if pos.x > 0 {
                        pos.x -= 1;
                    }
                }
                KeyCode::Char('5') => {
                    // stay
                }
                KeyCode::Char('6') => {
                    if pos.x + 1 < max_x {
                        pos.x += 1;
                    }
                }
                KeyCode::Char('7') => {
                    if pos.y > 0 && pos.x > 0 {
                        pos.y -= 1;
                        pos.x -= 1;
                    }
                }
                KeyCode::Char('8') => {
                    if pos.y > 0 {
                        pos.y -= 1;
                    }
                }
                KeyCode::Char('9') => {
                    if pos.y > 0 && pos.x + 1 < max_x {
                        pos.y -= 1;
                        pos.x += 1;
                    }
                }
                _ => {
                    // ignore
                    continue;
//                    println!("{:?}", code);
                }
            }
        }
        stdout.queue(
            cursor::MoveTo(pos.x, pos.y))?
            .queue(style::Print("X"))?;

        move_androids(&mut stdout, &mut androids, &pos);

        stdout.flush()?;
    }


//    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    println!("x={max_x} y={max_y}");

    crossterm::terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;

    Ok(())
}
