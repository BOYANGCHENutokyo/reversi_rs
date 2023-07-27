// https://github.com/kaisugi/reversi/blob/master/src/command_lexer.rs
// https://github.com/kaisugi/reversi/blob/master/src/command_parser.rs

use regex::Regex;
use crate::cmds::{Cmd, Res, Move, Color};

#[derive(PartialEq, Debug)]
pub enum Token {
    NL,
    INT(i32),
    OPEN,
    END,
    MOVE,
    START,
    ACK,
    BYE,
    WIN,
    LOSE,
    TIE,
    WHITE,
    BLACK,
    STR(String),
    EOF,
}

pub fn tokenize(input: &mut String, tokens: &mut Vec<Token>) {
    let p = input;
    let re = Regex::new(r"^-?\d+").unwrap();
    let re2 = Regex::new(r"^[^ \t\n\r]+").unwrap();

    while !(p.is_empty()) {
        if let Some(cap) = re.captures(p.as_str()) {
            let res = (&cap[0]).to_string();
            let n = res.len();
            let m: i32 = res.parse().unwrap();
            tokens.push(Token::INT(m));
            remove_times(p, n);
        } else {
            if p.starts_with(" ") || p.starts_with("\t") {
                remove_times(p, 1);
            } else if p.starts_with("\n") {
                tokens.push(Token::NL);
                remove_times(p, 1);
            } else if p.starts_with("OPEN") {
                tokens.push(Token::OPEN);
                remove_times(p, 4);
            } else if p.starts_with("END") {
                tokens.push(Token::END);
                remove_times(p, 3);
            } else if p.starts_with("MOVE") {
                tokens.push(Token::MOVE);
                remove_times(p, 4);
            } else if p.starts_with("START") {
                tokens.push(Token::START);
                remove_times(p, 5);
            } else if p.starts_with("ACK") {
                tokens.push(Token::ACK);
                remove_times(p, 3);
            } else if p.starts_with("BYE") {
                tokens.push(Token::BYE);
                remove_times(p, 3);
            } else if p.starts_with("WIN") {
                tokens.push(Token::WIN);
                remove_times(p, 3);
            } else if p.starts_with("LOSE") {
                tokens.push(Token::LOSE);
                remove_times(p, 4);
            } else if p.starts_with("TIE") {
                tokens.push(Token::TIE);
                remove_times(p, 3);
            } else if p.starts_with("WHITE") {
                tokens.push(Token::WHITE);
                remove_times(p, 5);
            } else if p.starts_with("BLACK") {
                tokens.push(Token::BLACK);
                remove_times(p, 5);
            } else {
                if let Some(cap) = re2.captures(p.as_str()) {
                    let res = (&cap[0]).to_string();
                    let n = res.len();
                    tokens.push(Token::STR(res));
                    remove_times(p, n);
                } else {
                    remove_times(p, 1);
                }
            }
        }
    }
    tokens.push(Token::EOF);
}

fn remove_times(s: &mut String, n: usize) {
    let s_tmp = &s[n..];
    *s = s_tmp.to_string();
}

pub fn parse(tokens: &mut Vec<Token>) -> Option<Cmd> {
    if tokens[0] == Token::OPEN {
        if let Token::STR(s) = &tokens[1] {
            let t = (*s).clone();
            Some(Cmd::Open(t))
        } else {
            panic!("Invalid Command");
        }
    } else if tokens[0] == Token::END {
        if let (Token::INT(m), Token::INT(n), Token::STR(s)) = (&tokens[2], &tokens[3], &tokens[4])
        {
            let t = (*s).clone();
            match tokens[1] {
                Token::WIN => Some(Cmd::End(Res::Win, (*m).clone(), (*n).clone(), t)),
                Token::LOSE => Some(Cmd::End(Res::Lose, (*m).clone(), (*n).clone(), t)),
                Token::TIE => Some(Cmd::End(Res::Tie, (*m).clone(), (*n).clone(), t)),
                _ => panic!("Invalid Command"),
            }
        } else {
            panic!("Invalid Command");
        }
    } else if tokens[0] == Token::MOVE {
        if let Token::STR(s) = &tokens[1] {
            let t = (*s).clone();
            if t == String::from("PASS") {
                Some(Cmd::Move(Move::Pass))
            } else if t.len() == 2 {
                let s0 = t.chars().nth(0).unwrap();
                let s1 = t.chars().nth(1).unwrap();

                if s0 >= 'A' && s0 <= 'H' && s1 >= '1' && s1 <= '8' {
                    Some(Cmd::Move(Move::To(
                        (s0 as i32) - ('A' as i32) + 1,
                        (s1 as i32) - ('1' as i32) + 1,
                    )))
                } else {
                    panic!("Invalid Command");
                }
            } else {
                panic!("Invalid Command");
            }
        } else {
            panic!("Invalid Command");
        }
    } else if tokens[0] == Token::START {
        if let (Token::STR(s), Token::INT(n)) = (&tokens[2], &tokens[3]) {
            let t = (*s).clone();
            match tokens[1] {
                Token::WHITE => Some(Cmd::Start(Color::White, t, (*n).clone())),
                Token::BLACK => Some(Cmd::Start(Color::Black, t, (*n).clone())),
                _ => panic!("Invalid Command"),
            }
        } else {
            panic!("Invalid Command");
        }
    } else if tokens[0] == Token::ACK {
        if let Token::INT(n) = &tokens[1] {
            Some(Cmd::Ack((*n).clone()))
        } else {
            panic!("Invalid Command");
        }
    } else if tokens[0] == Token::BYE {
        tokens.remove(0);
        Some(Cmd::Bye(render_scores(tokens)))
    } else if tokens[0] == Token::NL || tokens[0] == Token::EOF {
        None
    } else {
        panic!("Invalid Command");
    }
}

fn render_scores(tokens: &mut Vec<Token>) -> Vec<(String, (i32, i32, i32))> {
    let mut score_v = Vec::new();
    while tokens.len() >= 4 {
        if let (Token::STR(s), Token::INT(n1), Token::INT(n2), Token::INT(n3)) =
            (&tokens[0], &tokens[1], &tokens[2], &tokens[3]) {
            let t = (*s).clone();
            score_v.push((t, ((*n1).clone(), (*n2).clone(), (*n3).clone())));
            for _ in 0..4 {
                tokens.remove(0);
            }
        } else {
            panic!("Invalid Command");
        }
    }
    score_v
}