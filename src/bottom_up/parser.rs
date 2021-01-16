// #![cfg(feature = "alloc")] // needed for separated_list0
use super::Val;
use nom::{
  IResult, Parser,
  error::{ParseError},
  character::complete::{multispace0,digit1},
  bytes::complete::{tag},
  sequence::{delimited},
  combinator::{map_res,all_consuming},
  multi::{separated_list0},
  branch::{alt},
};



// combinator that trims whitespace before and after
fn trim<'a,O,E,P>(parser:P) -> impl FnMut(&'a str) -> IResult<&'a str,O,E> where
P: Parser<&'a str,O,E>,
E: ParseError<&'a str>,
{
   delimited(multispace0,parser,multispace0)
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

fn parse_val_intlist(input: &str) -> IResult<&str,Val> {
  trim( // trim whitespace
    delimited( // [...]
      tag("["),
      separated_list0(tag(","),parse_i32), // note parse_i32 handles trimming internal space
      tag("]")
    )
  )(input).map(|(rest,v)|(rest,Val::IntList(v)))
}

fn parse_val(input: &str) -> IResult<&str,Val> {
  alt((parse_val_intlist,parse_val_int))(input)
}
pub fn parse_val_complete(input: &str) -> IResult<&str,Val> {
  all_consuming(parse_val)(input) // errors if full input isn't consumed
}


#[test]
fn parser_test() {
  assert_eq!(parse_i32("12"),Ok(("",12i32)));
  assert_eq!(parse_val_int("  12 "),Ok(("",Val::Int(12))));
  assert_eq!(parse_val_intlist(" [12] "),Ok(("",Val::IntList(vec![12]))));
  assert_eq!(parse_val_intlist(" [12,13,14] "),Ok(("",Val::IntList(vec![12,13,14]))));
  assert_eq!(parse_val_intlist(" [] "),Ok(("",Val::IntList(vec![]))));
}



