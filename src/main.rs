/* Author: Martin Bajzek [M.Bajzek@gsi.de] */
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use std::{collections::HashSet, fs};
use regex::Regex;
use pest::{Parser, iterators::{Pair, Pairs}};
use pest_derive::Parser;
use std::convert::From;


#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ParspeccParser;

mod parser;
mod structures;

use crate::parser::*;
use crate::structures::*;

/* [1] : Types describing the unpack structures (not events/subevents) of Parspecc */
trait LMDHeader {}

#[derive(Debug,Clone)]
enum SubeventHeader {
    Type(u16),
    Subtype(u16),
    Procid(u16),
    Subcrate(u8),
    Control(u8),
} impl LMDHeader for SubeventHeader {}

#[derive(Debug,Clone)]
enum EventHeader {
    Type(u16),
    Subtype(u16),
    TriggerType(u16),
} impl LMDHeader for EventHeader {}

/* [Sub]Event statements can only contain the statements:
 *      name = struct_type( ((header_value = XY),* ) 
 * */

 /* Type T is generic over two types: EventHeader or SubeventHeader. */
#[derive(Debug,Clone)]
struct LMDState<T: LMDHeader> {
    struct_name: String,
    struct_type: Spanned<String>,
    header_spec: HashSet<T>,
}

type SubeventStatement = LMDState<SubeventHeader>;
type EventStatement = LMDState<EventHeader>;

#[derive(Debug,Clone)]
struct ParspeccSubevent {
    level: u32,
    name: String,

    // Inside the body:
    statements: Vec<SubeventStatement>,
}

#[derive(Debug,Clone)]
struct ParspeccEvent {
    level: u32,
    
    // Inside the body:
    statements: Vec<EventStatement>,
    ignore_unknown: bool,
}

/* In the spec file, three possible type declarations can be encountered:
 * -- Structs
 * -- Subevents
 * -- Events
 * First initial parser shall return a Vec<ObjectBlock> */
#[derive(Debug,Clone)]
enum ObjectBlock {
    Struct(String),
    Subevent()
}
#[derive(Debug,Clone)]
enum ParspeccObject {
    Struct(ParspeccStruct),
    Subevent(ParspeccSubevent),
    Event(ParspeccEvent),
}

fn mark_sub_events(input: &str) -> String {
    Regex::new(r#"(^|\s)((?:SUB)?EVENT\W)"#)
        .unwrap()
        .replace_all(input, "${1}@${2}")
        .to_string()
}

use pest::error::Error;
use pest::error::ErrorVariant;


fn parse_structure(input: Pair<'_, Rule>) -> ParspeccStruct {
    let mut inner = input.into_inner().peekable();
    let mut s : ParspeccStruct = Default::default();
    let p = inner.next().unwrap();
    s.name = make_spanned(p.as_str().into(), p.as_span());
    println!("Name: {}, span: {:?}", s.name.0, s.name.1);

    /* Parse possible params. */
    let params: Vec<&str> = match inner.peek() {
        Some(x) => {
            match x.as_rule() {
                Rule::IDENT_LIST => {
                    let mut idents = x.clone().into_inner();
                    inner.next();
                    idents.map(|x| x.as_str()).collect()
                },
                _ => vec![],
            }
        }
        _ => vec![],
    };
    s.params = params.into_iter().map(|x| String::from(x)).collect();

    /* Parse inside: MEMBER / BASIC / COMPOUND / ENCODE / FOR / DYN */ 
    while let Some(p) = inner.next() {
        match p.as_rule() {
            Rule::MEMBER => parse_member(&mut s, p),
            Rule::BASIC => parse_basic(&mut s, p),
            _ => {},
        }
    }
    s
}

fn parse_spec_file(input: &str) {
    //use pest::iterators::*;
    
    let mut parse_results = match ParspeccParser::parse(Rule::file, &input) {
        Err(e) => {
            println!("Parsing error: {}", e);
            println!("Parsing error location: {:?}", e.location);
            println!("Parsing full error: {:?}", e);
            match e.variant {
                ErrorVariant::CustomError { message } => {
                    println!("With message: {}", message);
                },
                _ => {}, 
            }
            panic!("Aborting");
        },
        Ok(s) => s,
    };

    let structs = parse_results
        .next()
        .unwrap();
    for m in structs.into_inner() {
        match m.as_rule() {
            Rule::r#struct => {
                parse_structure(m);
            },
            Rule::subevent => {
                println!("Found subevent!");
                let mut inner = m.into_inner();
                let struct_name = inner.next().unwrap().as_str();
                println!("Subevent name: {}", struct_name);
            },
            Rule::event => {
                println!("Found event!");
                let mut inner = m.into_inner();
                let trig_type: u32 = match inner.next() {
                    Some(x) => x.as_str().parse().unwrap_or(1),
                    None => 1,
                };
                println!(" .. trigger type: {}\n", trig_type);
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
fn main() {
    let _file_name = "test.spec";
    let _file = fs::read_to_string(_file_name).expect("Cannot open file.");
  
    pest::set_error_detail(true);
    /* [1] Match all the SUBEVENT(..) and EVENT.. declaration and tag it with '@'
     * This is the input to the parser. */
    // let input = mark_sub_events(&_file);
    
    parse_spec_file(&_file);
    println!("\nEnd of main..\n");
}
