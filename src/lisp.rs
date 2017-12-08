use std::str::FromStr;
use ast::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Exprs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23(&'input str),
        TermNumRegex(&'input str),
        TermTermRegex(&'input str),
        Nt_28_3cExpr_3e_29(Ast),
        Nt_28_3cExpr_3e_29_2a(::std::vec::Vec<Ast>),
        Nt_28_3cExpr_3e_29_2b(::std::vec::Vec<Ast>),
        NtExpr(Ast),
        NtExprs(Vec<Ast>),
        NtNum(i32),
        NtTerm(String),
        Nt____Exprs(Vec<Ast>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        7, 0, 8, 9, 10,
        // State 1
        7, -11, 8, 9, 10,
        // State 2
        -4, -4, -4, -4, -4,
        // State 3
        -14, -14, -14, -14, -14,
        // State 4
        -7, -7, -7, -7, -7,
        // State 5
        -9, -9, -9, -9, -9,
        // State 6
        7, -10, 8, 9, 10,
        // State 7
        -8, -8, -8, -8, -8,
        // State 8
        -12, -12, -12, -12, -12,
        // State 9
        -13, -13, -13, -13, -13,
        // State 10
        -5, -5, -5, -5, -5,
        // State 11
        0, 13, 0, 0, 0,
        // State 12
        -6, -6, -6, -6, -6,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -10,
        -11,
        -4,
        -14,
        -7,
        -9,
        0,
        -8,
        -12,
        -13,
        -5,
        0,
        -6,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 3, 4, 5, 6, 0,
        // State 1
        0, 0, 0, 11, 0, 5, 6, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 2, 3, 12, 5, 6, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"r#"\"(?:[^\"\\\\]|\\\\.)*\""#"###,
            r###"NumRegex"###,
            r###"TermRegex"###,
        ];
        __ACTION[(__state * 5)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Exprs<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Ast>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (1, _) if true => 0,
                (2, _) if true => 1,
                (0, _) if true => 2,
                (4, _) if true => 3,
                (3, _) if true => 4,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 5 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::TermNumRegex((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::TermTermRegex((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Ast>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr>) = Expr => ActionFn(10);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr>)* =  => ActionFn(8);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action8::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr>)* = (<Expr>)+ => ActionFn(9);
                let __sym0 = __pop_Nt_28_3cExpr_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr>)+ = Expr => ActionFn(13);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr>)+ = (<Expr>)+, Expr => ActionFn(14);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_29_2b(__nt), __end));
                2
            }
            6 => {
                // Expr = "(", Exprs, ")" => ActionFn(2);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExprs(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                3
            }
            7 => {
                // Expr = Num => ActionFn(3);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                3
            }
            8 => {
                // Expr = r#"\"(?:[^\"\\\\]|\\\\.)*\""# => ActionFn(4);
                let __sym0 = __pop_Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                3
            }
            9 => {
                // Expr = Term => ActionFn(5);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                3
            }
            10 => {
                // Exprs =  => ActionFn(15);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action15::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExprs(__nt), __end));
                4
            }
            11 => {
                // Exprs = (<Expr>)+ => ActionFn(16);
                let __sym0 = __pop_Nt_28_3cExpr_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprs(__nt), __end));
                4
            }
            12 => {
                // Num = NumRegex => ActionFn(7);
                let __sym0 = __pop_TermNumRegex(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                5
            }
            13 => {
                // Term = TermRegex => ActionFn(6);
                let __sym0 = __pop_TermTermRegex(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                6
            }
            14 => {
                // __Exprs = Exprs => ActionFn(0);
                let __sym0 = __pop_NtExprs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 8 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermNumRegex<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermNumRegex(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermTermRegex<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermTermRegex(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ast, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ast>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ast>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ast, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ast>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Exprs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ast>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Exprs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Exprs::parse_Exprs;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:\")(?:(?u:[\u{0}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\")",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:[-\\*-\\+\\.-/<-\\?A-Z_-_a-z])+(?u:[-\\*-\\+\\.-9<-\\?A-Z_-_a-z])*",
                "^(?u:[-\\+-\\+])?(?u:[0-9])+",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\")(?:(?u:[\u{0}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\")").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:[-\\*-\\+\\.-/<-\\?A-Z_-_a-z])+(?u:[-\\*-\\+\\.-9<-\\?A-Z_-_a-z])*").unwrap(),
                __regex::Regex::new("^(?u:[-\\+-\\+])?(?u:[0-9])+").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 5 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Ast>, usize),
) -> Vec<Ast>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ast>, usize),
) -> Vec<Ast>
{
    v
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Ast>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Ast
{
    Ast::List(__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> Ast
{
    Ast::Int(__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Ast
{
    {
      let val = __0;
      Ast::Str(val[1..val.len() - 1].to_owned())
    }
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Ast
{
    Ast::Atom(__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    (__0).to_owned()
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Ast>
{
    vec![]
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ast>, usize),
) -> ::std::vec::Vec<Ast>
{
    v
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Ast, usize),
) -> Ast
{
    (__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Ast, usize),
) -> ::std::vec::Vec<Ast>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ast>, usize),
    (_, e, _): (usize, Ast, usize),
) -> ::std::vec::Vec<Ast>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    __0: (usize, Ast, usize),
) -> ::std::vec::Vec<Ast>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action10(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ast>, usize),
    __1: (usize, Ast, usize),
) -> ::std::vec::Vec<Ast>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action10(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Ast>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action8(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ast>, usize),
) -> Vec<Ast>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action9(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
