use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};
use clap::Parser;
use tailcall::tailcall;

mod cmds;
use cmds::{Color, Cmd, move_to_string, idx_to_move, move_to_idx, Res};
mod parse;
use parse::{tokenize, parse};
mod bitboard;
use bitboard::Board;
mod search;
use search::search;

/// Reversi Command Line Interface
/// Author: Luhao Liu <luhao.liu@a.riken.jp>
#[derive(Parser)]
struct Args {
    /// Hostname
    #[arg(short = 'H', default_value = "localhost")]
    hostname: String,

    /// Port
    #[arg(short = 'p', default_value = "3000")]
    port: u16,

    /// Player Name
    #[arg(short = 'n')]
    player: String,
}

enum State {
    WaitingStart,
    MyMove,
    OpMove,
    WaitingAck,
}

fn write_cmd(writer: &mut BufWriter<&TcpStream>, cmd: Cmd) {
    let mut send_msg = |msg: String| {
        writer.write(msg.as_bytes()).expect("Send Error!");
        writer.flush().unwrap();
        #[cfg(debug_assertions)]
        print!("Sent: {}", msg);
    };

    match cmd {
        Cmd::Move(mv) => {
            let msg = format!("MOVE {}\n", move_to_string(mv));
            send_msg(msg);
        }
        Cmd::Open(s) => {
            let msg = format!("OPEN {}\n", s);
            send_msg(msg);
        }
        _ => {
            panic!("Illegal Send Message!");
        }
    }
}

fn read_cmd(reader: &mut BufReader<&TcpStream>) -> Cmd {
    let mut msg = String::new();
    reader.read_line(&mut msg).expect("Receive Error!");
    #[cfg(debug_assertions)]
    print!("Received: {}", &msg);
    let mut tokens = Vec::new();
    tokenize(&mut msg, &mut tokens);
    match parse(&mut tokens) {
        Some(cmd) => {
            cmd
        }
        None => read_cmd(reader)
    }
}

fn print_scores(scores: Vec<(String, (i32, i32, i32))>) {
    for (a, (i, j, k)) in scores {
        println!("{}: {} (Win {}, Lose {})", a, i, j, k);
    }
}

#[tailcall]
fn game(
    state: State,
    reader: &mut BufReader<&TcpStream>,
    writer: &mut BufWriter<&TcpStream>,
    color: Color,
    board: &mut Board,
    oppo_name: String,
    no_time: bool,
) {
    const DEFAULT_DEPTH: usize = 11;
    const REDUCED_DEPTH: usize = 8;
    match state {
        State::WaitingStart => match read_cmd(reader) {
            Cmd::Bye(scores) => {
                print_scores(scores);
            }
            Cmd::Start(color, oppo_name, _) => {
                match color {
                    Color::Black => {
                        game(
                        State::MyMove,
                        reader,
                        writer,
                        Color::Black,
                        board,
                        oppo_name,
                        no_time
                        )
                    },
                    Color::White => {
                        game(
                            State::OpMove,
                            reader,
                            writer,
                            Color::White,
                            board,
                            oppo_name,
                            no_time
                        )
                    }
                    _ => {
                        panic!("Invalid Command");
                    }
                }
            }
            _ => {
                panic!("Invalid Command");
            }
        },
        State::MyMove => {
            let (mv, hints) = search(board, if no_time {REDUCED_DEPTH} else {DEFAULT_DEPTH});
            write_cmd(writer, Cmd::Move(idx_to_move(&mv)));
            if mv != 0 {
                board.next(mv, hints);
            }
            #[cfg(debug_assertions)]
            board.print();
            board.exchange();
            game(
                State::WaitingAck,
                reader,
                writer,
                color,
                board,
                oppo_name,
                no_time
            )
        },
        State::OpMove => {
            match read_cmd(reader) {
                Cmd::Move(mv) => {
                    let (_, hints) = board.legals();
                    if move_to_idx(&mv) != 0 {
                        board.next(move_to_idx(&mv), hints);
                    }
                    board.exchange();
                    #[cfg(debug_assertions)]
                    board.print();
                    game(
                        State::MyMove,
                        reader,
                        writer,
                        color,
                        board,
                        oppo_name,
                        no_time
                    )
                },
                Cmd::End(res, n, m, r) => {
                    match res {
                        Res::Win => println!("You Win. ({} vs {}), {}", n, m, r),
                        Res::Lose => println!("You Lose. ({} vs {}), {}", n, m, r),
                        Res::Tie => println!("Draw. ({} vs {}), {}", n, m, r)
                    };
                    board.clear();
                    game(
                        State::WaitingStart,
                        reader,
                        writer,
                        Color::Empty,
                        board,
                        oppo_name,
                        false
                    )
                },
                _ => {
                    panic!("Invalid Command");
                }
            }
        },
        State::WaitingAck => {
            match read_cmd(reader) {
                Cmd::Ack(time) => {
                    game(
                        State::OpMove,
                        reader,
                        writer,
                        color,
                        board,
                        oppo_name,
                        if time < 30000 {true} else {false}
                    )
                },
                Cmd::End(res, n, m, r) => {
                    match res {
                        Res::Win => println!("You Win. ({} vs {}), {}", n, m, r),
                        Res::Lose => println!("You Lose. ({} vs {}), {}", n, m, r),
                        Res::Tie => println!("Draw. ({} vs {}), {}", n, m, r)
                    };
                    board.clear();
                    game(
                        State::WaitingStart,
                        reader,
                        writer,
                        Color::Empty,
                        board,
                        oppo_name,
                        false
                    )
                },
                _ => {
                    panic!("Invalid Command");
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("Player Name: {}", &args.player);

    let addr = (args.hostname.clone(), args.port).to_socket_addrs().unwrap()
        .find(|x| (*x).is_ipv4()).unwrap();
    let stream = TcpStream::connect(addr).unwrap();
    println!("Successfully connected to {}:{}.", &args.hostname, &args.port);
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    write_cmd(&mut writer, Cmd::Open(args.player.clone()));
    game(
        State::WaitingStart,
        &mut reader,
        &mut writer,
        Color::Empty,
        &mut Board::new(),
        args.player.clone(),
        false
    );
}
