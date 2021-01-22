// #![cfg(feature = "alloc")] // needed for separated_list0
use super::{Val};
use nom::{
  IResult, Parser,
  error::{ParseError},
  character::complete::{multispace0,digit1},
  bytes::complete::{tag},
  sequence::{delimited,separated_pair},
  combinator::{map_res,all_consuming},
  multi::{separated_list0},
  branch::{alt},
};

#[derive(Debug,PartialEq,Eq)]
pub struct ParseValErr(pub String);
pub type ParseResult<T> = Result<T,ParseValErr>;



// combinator that trims whitespace before and after
fn trim<'a,O,E,P>(parser:P) -> impl FnMut(&'a str) -> IResult<&'a str,O,E> where
P: Parser<&'a str,O,E>,
E: ParseError<&'a str>,
{
   delimited(multispace0,parser,multispace0)
}
// combinator that ensures that the whole input is consumed, throws out the
// `rest` empty string, and transforms the error
fn complete<'a,O,P>(parser:P, input: &'a str) -> ParseResult<O> where
P: Parser<&'a str,O,nom::error::Error<&'a str>>,
{
  all_consuming(parser)(input) // errors if full input isn't consumed
    .map(|(_,v)|v)
    .map_err(|e|ParseValErr(e.to_string()))
}

fn get_i32(input:&str) -> Result<i32,std::num::ParseIntError> {
  input.parse()
}

fn parse_i32(input:&str) -> IResult<&str,i32> {
  map_res( // its a bit like "map" on a result. If digit1 fails we abort. if it succeeds we apply get_i32 to the result and if that fails we abort (autoconverting error to IResult)
    digit1, // parser to grab 1+ digits
    get_i32 // function that may throw error, but if it does we autoconvert the error (with trait FromExternalError)
  )(input)
}

// trims leading/training whitespace then parses int a Val::Int
fn parse_val_int(input: &str) -> IResult<&str,Val> {
  trim(parse_i32)(input).map(|(rest,v)|(rest,Val::Int(v)))
}

// trims leading/training whitespace then parses int a Val::IntList
fn parse_val_intlist(input: &str) -> IResult<&str,Val> {
  trim( // trim whitespace
    delimited( // [...]
      tag("["),
      separated_list0(tag(","),parse_i32), // note parse_i32 handles trimming internal space
      tag("]")
    )
  )(input).map(|(rest,v)|(rest,Val::IntList(v)))
}

// trims leading/training whitespace then parses int a Val::IntList or Val::Int
fn parse_val(input: &str) -> IResult<&str,Val> {
  alt((parse_val_intlist,parse_val_int))(input)
}

// parse a series of comma separated values into a Vec<Val>. Flexible with whitespace.
fn parse_vals(input: &str) -> IResult<&str,Vec<Val>> {
  separated_list0(tag(","),parse_val)(input)
}

// parse an io example written like "[1,2,3,4], 7 -> [1,2,3]" where
// the left side has a comma sep list of values (`parse_vals`) and
// right side has a single value. Returns an (inputs,output) tuple
fn parse_io(input: &str) -> IResult<&str,(Vec<Val>,Val)> {
  separated_pair(parse_vals, tag("->"), parse_val)(input)
}


pub fn parse_val_complete(input: &str) -> ParseResult<Val> {
  complete(parse_val,input) // errors if full input isn't consumed
}

pub fn parse_io_complete(input: &str) -> ParseResult<(Vec<Val>,Val)> {
  complete(parse_io,input)
}


#[test]
fn parser_test() {
  use Val::*;
  assert_eq!(parse_i32("12"),Ok(("",12i32)));
  assert_eq!(parse_val_int("  12 "),Ok(("",Int(12))));
  assert_eq!(parse_val_intlist(" [12] "),Ok(("",IntList(vec![12]))));
  assert_eq!(parse_val_intlist(" [12,13,14] "),Ok(("",IntList(vec![12,13,14]))));
  assert_eq!(parse_val_intlist(" [] "),Ok(("",IntList(vec![]))));
  assert_eq!(parse_vals(" [12,13,14]  ,  12 "),Ok(("",vec![IntList(vec![12,13,14]),Int(12)])));
  assert_eq!(parse_io(" [12,13,14]  ,  12 ->  56"),Ok(("",(vec![IntList(vec![12,13,14]),Int(12)],Int(56)))));
  assert_eq!(parse_io_complete(" [12,13,14]  ,  12 ->  56"),Ok((vec![IntList(vec![12,13,14]),Int(12)],Int(56))));
  // assert_eq!(parse_val_complete(" [12,13,14] "),Ok(IntList(vec![12,13,14])));
}



