// Generated by rust-peg. Do not edit.
#![allow(non_snake_case, unused)]
use super::super::{ast, runtime};
use self::RuleResult::{Matched, Failed};
fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
fn char_range_at(s: &str, pos: usize) -> (char, usize) {
    let c = &s[pos..].chars().next().unwrap();
    let next_pos = pos + c.len_utf8();
    (*c, next_pos)
}
#[derive(Clone)]
enum RuleResult<T> { Matched(usize, T), Failed, }
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub expected: ::std::collections::HashSet<&'static str>,
}
pub type ParseResult<T> = Result<T, ParseError>;
impl ::std::fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter)
     -> ::std::result::Result<(), ::std::fmt::Error> {
        try!(write ! (
             fmt , "error at {}:{}: expected " , self . line , self . column
             ));
        if self.expected.len() == 1 {
            try!(write ! (
                 fmt , "`{}`" , escape_default (
                 self . expected . iter (  ) . next (  ) . unwrap (  ) ) ));
        } else {
            let mut iter = self.expected.iter();
            try!(write ! (
                 fmt , "one of `{}`" , escape_default (
                 iter . next (  ) . unwrap (  ) ) ));
            for elem in iter {
                try!(write ! ( fmt , ", `{}`" , escape_default ( elem ) ));
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for ParseError {
    fn description(&self) -> &str { "parse error" }
}
fn slice_eq(input: &str, state: &mut ParseState, pos: usize, m: &'static str)
 -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    let l = m.len();
    if input.len() >= pos + l &&
           &input.as_bytes()[pos..pos + l] == m.as_bytes() {
        Matched(pos + l, ())
    } else { state.mark_failure(pos, m) }
}
fn slice_eq_case_insensitive(input: &str, state: &mut ParseState, pos: usize,
                             m: &'static str) -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    let mut used = 0usize;
    let mut input_iter = input[pos..].chars().flat_map(|x| x.to_uppercase());
    for m_char_upper in m.chars().flat_map(|x| x.to_uppercase()) {
        used += m_char_upper.len_utf8();
        let input_char_result = input_iter.next();
        if input_char_result.is_none() ||
               input_char_result.unwrap() != m_char_upper {
            return state.mark_failure(pos, m);
        }
    }
    Matched(pos + used, ())
}
fn any_char(input: &str, state: &mut ParseState, pos: usize)
 -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    if input.len() > pos {
        let (_, next) = char_range_at(input, pos);
        Matched(next, ())
    } else { state.mark_failure(pos, "<character>") }
}
fn pos_to_line(input: &str, pos: usize) -> (usize, usize) {
    let mut remaining = pos;
    let mut lineno: usize = 1;
    for line in input.lines() {
        let line_length = line.len() + 1;
        if remaining < line_length { return (lineno, remaining + 1); }
        remaining -= line_length;
        lineno += 1;
    }
    return (lineno, remaining + 1);
}
struct ParseState {
    max_err_pos: usize,
    expected: ::std::collections::HashSet<&'static str>,
}
impl ParseState {
    fn new() -> ParseState {
        ParseState{max_err_pos: 0,
                   expected: ::std::collections::HashSet::new(),}
    }
    fn mark_failure(&mut self, pos: usize, expected: &'static str)
     -> RuleResult<()> {
        if pos > self.max_err_pos {
            self.max_err_pos = pos;
            self.expected.clear();
        }
        if pos == self.max_err_pos { self.expected.insert(expected); }
        Failed
    }
}
fn parse_block<'input>(input: &'input str, state: &mut ParseState, pos: usize)
 -> RuleResult<ast::Block> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res = parse_statement(input, state, pos);
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    Matched(repeat_pos, repeat_value)
                };
            match seq_res {
                Matched(pos, s) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { s })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_statement<'input>(input: &'input str, state: &mut ParseState,
                           pos: usize) -> RuleResult<ast::Statement> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = parse_expression(input, state, pos);
                    match seq_res {
                        Matched(pos, e) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos,
                                        { ast::Statement::Expression(e) })
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, ";");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { ast::Statement::Empty })
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_expression<'input>(input: &'input str, state: &mut ParseState,
                            pos: usize) -> RuleResult<ast::Expression> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = parse_expression(input, state, pos);
                    match seq_res {
                        Matched(pos, f) => {
                            {
                                let seq_res =
                                    slice_eq(input, state, pos, "(");
                                match seq_res {
                                    Matched(pos, _) => {
                                        {
                                            let seq_res =
                                                parse_expression_list(input,
                                                                      state,
                                                                      pos);
                                            match seq_res {
                                                Matched(pos, a) => {
                                                    {
                                                        let seq_res =
                                                            slice_eq(input,
                                                                     state,
                                                                     pos,
                                                                     ")");
                                                        match seq_res {
                                                            Matched(pos, _) =>
                                                            {
                                                                {
                                                                    let match_str =
                                                                        &input[start_pos..pos];
                                                                    Matched(pos,
                                                                            {
                                                                                ast::Expression::Call(Box::new(f),
                                                                                                      a)
                                                                            })
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let choice_res =
                    {
                        let start_pos = pos;
                        {
                            let seq_res = parse_expression(input, state, pos);
                            match seq_res {
                                Matched(pos, o) => {
                                    {
                                        let seq_res =
                                            slice_eq(input, state, pos, ".");
                                        match seq_res {
                                            Matched(pos, _) => {
                                                {
                                                    let seq_res =
                                                        parse_identifier(input,
                                                                         state,
                                                                         pos);
                                                    match seq_res {
                                                        Matched(pos, m) => {
                                                            {
                                                                let match_str =
                                                                    &input[start_pos..pos];
                                                                Matched(pos,
                                                                        {
                                                                            ast::Expression::Member(Box::new(o),
                                                                                                    m)
                                                                        })
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                    };
                match choice_res {
                    Matched(pos, value) => Matched(pos, value),
                    Failed => {
                        let choice_res =
                            {
                                let start_pos = pos;
                                {
                                    let seq_res =
                                        parse_identifier(input, state, pos);
                                    match seq_res {
                                        Matched(pos, i) => {
                                            {
                                                let match_str =
                                                    &input[start_pos..pos];
                                                Matched(pos,
                                                        {
                                                            ast::Expression::Identifier(i)
                                                        })
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            };
                        match choice_res {
                            Matched(pos, value) => Matched(pos, value),
                            Failed => {
                                let start_pos = pos;
                                {
                                    let seq_res =
                                        parse_literal(input, state, pos);
                                    match seq_res {
                                        Matched(pos, v) => {
                                            {
                                                let match_str =
                                                    &input[start_pos..pos];
                                                Matched(pos,
                                                        {
                                                            ast::Expression::Literal(v)
                                                        })
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn parse_expression_list<'input>(input: &'input str, state: &mut ParseState,
                                 pos: usize)
 -> RuleResult<ast::ExpressionList> {
    {
        let mut repeat_pos = pos;
        let mut repeat_value = vec!();
        loop  {
            let pos = repeat_pos;
            let pos =
                if repeat_value.len() > 0 {
                    let sep_res = slice_eq(input, state, pos, ",");
                    match sep_res {
                        Matched(newpos, _) => { newpos }
                        Failed => break ,
                    }
                } else { pos };
            let step_res = parse_expression(input, state, pos);
            match step_res {
                Matched(newpos, value) => {
                    repeat_pos = newpos;
                    repeat_value.push(value);
                }
                Failed => { break ; }
            }
        }
        Matched(repeat_pos, repeat_value)
    }
}
fn parse_literal<'input>(input: &'input str, state: &mut ParseState,
                         pos: usize) -> RuleResult<runtime::Value> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_integer(input, state, pos);
            match seq_res {
                Matched(pos, i) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { runtime::Value::Number(i) })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_whitespace<'input>(input: &'input str, state: &mut ParseState,
                            pos: usize) -> RuleResult<()> {
    {
        let mut repeat_pos = pos;
        let mut repeat_value = vec!();
        loop  {
            let pos = repeat_pos;
            let step_res =
                if input.len() > pos {
                    let (ch, next) = char_range_at(input, pos);
                    match ch {
                        ' ' | '\t' | '\r' | '\n' => Matched(next, ()),
                        _ => state.mark_failure(pos, "[ \t\r\n]"),
                    }
                } else { state.mark_failure(pos, "[ \t\r\n]") };
            match step_res {
                Matched(newpos, value) => {
                    repeat_pos = newpos;
                    repeat_value.push(value);
                }
                Failed => { break ; }
            }
        }
        if repeat_value.len() >= 1usize {
            Matched(repeat_pos, ())
        } else { Failed }
    }
}
fn parse_identifier<'input>(input: &'input str, state: &mut ParseState,
                            pos: usize) -> RuleResult<String> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res =
                            if input.len() > pos {
                                let (ch, next) = char_range_at(input, pos);
                                match ch {
                                    'a' ...'z' | 'A' ...'Z' | '_' =>
                                    Matched(next, ()),
                                    _ => state.mark_failure(pos, "[a-zA-Z_]"),
                                }
                            } else { state.mark_failure(pos, "[a-zA-Z_]") };
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    if repeat_value.len() >= 1usize {
                        Matched(repeat_pos, ())
                    } else { Failed }
                };
            match seq_res {
                Matched(pos, _) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { match_str.to_string() })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_integer<'input>(input: &'input str, state: &mut ParseState,
                         pos: usize) -> RuleResult<f64> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res =
                            if input.len() > pos {
                                let (ch, next) = char_range_at(input, pos);
                                match ch {
                                    '0' ...'9' => Matched(next, ()),
                                    _ => state.mark_failure(pos, "[0-9]"),
                                }
                            } else { state.mark_failure(pos, "[0-9]") };
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    if repeat_value.len() >= 1usize {
                        Matched(repeat_pos, ())
                    } else { Failed }
                };
            match seq_res {
                Matched(pos, _) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { match_str.parse().unwrap() })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
pub fn block<'input>(input: &'input str) -> ParseResult<ast::Block> {
    let mut state = ParseState::new();
    match parse_block(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
