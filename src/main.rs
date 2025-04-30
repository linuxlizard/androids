/* davep 20250325 ; convert from ncurses to crossterm */

use rand::Rng;
use std::{thread, time::Duration};
//use crate::line;
use std::io::{self, Write};
// for logging
use std::net::UdpSocket;

use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Print},
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
                ).unwrap();

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

fn step_enemy(logger: &std::net::UdpSocket, android: &mut Point, player: &Point) {
    // using the algorithm from Robots in BSDGames
    // https://github.com/vattam/BSDGames.git
    log_message(&logger, "step_enemy\n");

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
                ).unwrap();
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

fn is_player_dead(stdout: &mut io::Stdout, androids: &Vec<Point>, player: &Point ) -> bool
{
    let mut dead:bool = false;

    for a in androids.iter() {
        if a.x == player.x && a.y == player.y {
            dead = true;
            break;
        }
    }

    if dead {
        stdout.queue(
            cursor::MoveTo(player.x, player.y)).unwrap()
            .queue(style::Print("#")).unwrap();
    }

    return dead;
}

fn move_androids(logger: &std::net::UdpSocket, stdout: &mut io::Stdout, androids: &mut Vec<Point>, trash: &mut Vec<Point>, player: &Point)
{
    let mut dead:bool;

    log_message(&logger, "move_androids\n");

    for a in androids.iter_mut() {
        // erase
        stdout.queue(
            cursor::MoveTo(a.x, a.y)).unwrap()
            .queue(Print( " ")).unwrap();

        step_enemy(&logger, a, player);

        // if we have a collision with trash, mark self dead
        dead = false;
        for t in trash.iter() {
            let msg = format!("check trash a.x={} a.y={} t.x={} t.y={}\n", a.x, a.y, t.x, t.y);
            log_message(&logger, &msg);

            if a.x == t.x && a.y == t.y {
                let msg = format!("trash collision at {} {}\n", a.x, a.y);
                log_message(logger, &msg);

                a.x = u16::MAX;
                a.y = u16::MAX;
                dead = true;
                break;
            }
        }

        if !dead {
            // draw new
            stdout.queue(
                cursor::MoveTo(a.x, a.y)).unwrap()
                .queue(Print( "A")).unwrap();
        }
    }

    let _ = androids.retain(|a| a.x!=u16::MAX && a.y != u16::MAX);

    // check for collision with another android
    let msg = format!("len={}\n", androids.len());
    log_message(logger, &msg);

    if androids.len() == 0 {
        return;
    }

    for i in 0..androids.len()-1 {
        let (left, right) = androids.split_at_mut(i+1);

        let msg = format!("left={} right={}\n", left.len(), right.len());
        log_message(logger, &msg);

        let a = &mut left[left.len()-1];
        for b in right.iter_mut() {
            if a.x == b.x && a.y == b.y {
                let msg = format!("collision at {} {}\n", a.x, a.y);
                log_message(logger, &msg);

                stdout.queue(
                    cursor::MoveTo(a.x, a.y)).unwrap()
                    .queue(Print( "*")).unwrap();

                trash.push( Point{ x: a.x, y: a.y } );

                b.x = u16::MAX;
                b.y = u16::MAX;
                a.x = u16::MAX;
                a.y = u16::MAX;
            }
        }
    }

    let _ = androids.retain(|a| a.x!=u16::MAX && a.y != u16::MAX);

    let msg = format!("androids={}\n", androids.len());
    log_message(logger, &msg);
}

fn log_message(socket: &std::net::UdpSocket, msg:&str)
{
    socket.send_to( msg.as_bytes(), "127.0.0.1:9999").expect("udp send");
}


fn main() -> io::Result<()> 
{
    let logger = UdpSocket::bind("127.0.0.1:0").expect("udp bind");
    log_message(&logger, "Hello, world!\n");

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


    let mut trash: Vec<Point> = Vec::new();

    let mut androids: Vec<Point> = Vec::new();
    while androids.len() < 10 {
        let p = random_position(max_y, max_x);
        if p.x == pos.x && p.y == pos.y {
            // player position, roll again
            continue;
        }
        
        stdout.queue(
           cursor::MoveTo(p.x, p.y))?
            .queue(style::Print("A"))?;

        androids.push(p);
    }

    stdout.flush()?;

    loop {
        log_message(&logger, "waiting for player\n");

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

        log_message(&logger, "bottom of loop\n");

        stdout.queue(
            cursor::MoveTo(pos.x, pos.y))?
            .queue(style::Print("X"))?;

        move_androids(&logger, &mut stdout, &mut androids, &mut trash, &pos);

        let msg = format!("trash={}\n", trash.len());
        log_message(&logger, &msg);

        // did player collide with a droid?
        if is_player_dead(&mut stdout, &androids, &pos) {
            println!("you died");
            log_message(&logger, "androids win\n");
            break;
        }

        if androids.len() == 0 {
            println!("you win!");
            log_message(&logger, "player wins\n");
            break;
        }

        stdout.flush()?;
    }


//    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    println!("x={max_x} y={max_y}");

    crossterm::terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;

    Ok(())
}
