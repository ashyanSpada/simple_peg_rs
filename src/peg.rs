use crate::utils::*;

fn parse_sentence<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "sentence", input, pos);
    let key = format!("sentence::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

    if state.is_some() {
        state = parse_rule(memo, key.clone(), input, state.clone().unwrap().pos);

        let mut r = String::new();
        if state.is_some() {
            r = state.clone().unwrap().value;
        }

        if state.is_some() {
            stack.push(state.clone());
            let mut is_first = true;
            loop {
                state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

                if state.is_some() {
                    state = parse_rule(memo, key.clone(), input, state.clone().unwrap().pos);

                    if state.is_some() {}
                }

                if state.is_some() {
                    let last_state = stack.pop().unwrap();
                    let mut cur_state = state.clone().unwrap();
                    if !is_first && last_state.is_some() {
                        cur_state.value = last_state.unwrap().value + &cur_state.value;
                    }
                    stack.push(Some(cur_state));
                } else {
                    break;
                }
                is_first = false;
            }
            state = stack.pop().unwrap();

            let mut g = String::new();
            if state.is_some() {
                g = state.clone().unwrap().value;
            }

            if state.is_some() {
                state = Some(State {
                    value: (r + &g),
                    pos: state.clone().unwrap().pos,
                });
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_start<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "start", input, pos);
    let key = format!("start::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = parse_sentence(memo, key.clone(), input, state.clone().unwrap().pos);

    if state.is_some() {}

    memo.insert(key, state.clone());
    state
}

fn parse_sp<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "sp", input, pos);
    let key = format!("sp::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, " ");

    if state.is_some() {}

    if state.is_none() {
        state = stack.pop().unwrap();

        stack.push(state.clone());

        state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "\n");

        if state.is_some() {}

        if state.is_none() {
            state = stack.pop().unwrap();

            state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "\t");

            if state.is_some() {}
        } else {
            stack.pop().unwrap();
        }
    } else {
        stack.pop().unwrap();
    }

    memo.insert(key, state.clone());
    state
}

fn parse__<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "_", input, pos);
    let key = format!("_::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = parse_sp(memo, key.clone(), input, state.clone().unwrap().pos);

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {}
    }

    if state.is_none() {
        state = stack.pop().unwrap();
    } else {
        stack.pop().unwrap();
    }

    memo.insert(key, state.clone());
    state
}

fn parse_rule<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "rule", input, pos);
    let key = format!("rule::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = parse_name(memo, key.clone(), input, state.clone().unwrap().pos);

    let mut n = String::new();
    if state.is_some() {
        n = state.clone().unwrap().value;
    }

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "<-");

            if state.is_some() {
                state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

                if state.is_some() {
                    state = parse_choice(memo, key.clone(), input, state.clone().unwrap().pos);

                    let mut body = String::new();
                    if state.is_some() {
                        body = state.clone().unwrap().value;
                    }

                    if state.is_some() {
                        state = literal(memo, key.clone(), input, state.clone().unwrap().pos, ".");

                        if state.is_some() {
                            state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

                            if state.is_some() {
                                state = Some(State {
                                    value: (format!("
                        fn parse_{n}<'a,'b>(memo: &'a mut Memo<String>, parent: String, input: &'b str, pos: usize) -> Option<State<String>> {{
                            add_operation(memo, parent, \"{n}\", input, pos);
                            let key = format!(\"{n}::{{pos}}\");
                            let (mut state, existed) = memo.get(key.clone());
                            if existed {{
                                return state;
                            }}
                            state = Some(State{{value: \"\".to_string(), pos: pos}});
                            let mut stack: Vec<Option<State<String>>> = Vec::new();
                            {body}
                            memo.insert(key, state.clone());
                            state
                        }}
                        "
                    )
               ),
                                    pos: state.clone().unwrap().pos,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_meta<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "meta", input, pos);
    let key = format!("meta::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "!");

    if state.is_some() {}

    if state.is_none() {
        state = stack.pop().unwrap();

        stack.push(state.clone());

        state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "\'");

        if state.is_some() {}

        if state.is_none() {
            state = stack.pop().unwrap();

            stack.push(state.clone());

            state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "<-");

            if state.is_some() {}

            if state.is_none() {
                state = stack.pop().unwrap();

                stack.push(state.clone());

                state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "/");

                if state.is_some() {}

                if state.is_none() {
                    state = stack.pop().unwrap();

                    stack.push(state.clone());

                    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, ".");

                    if state.is_some() {}

                    if state.is_none() {
                        state = stack.pop().unwrap();

                        stack.push(state.clone());

                        state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "(");

                        if state.is_some() {}

                        if state.is_none() {
                            state = stack.pop().unwrap();

                            stack.push(state.clone());

                            state =
                                literal(memo, key.clone(), input, state.clone().unwrap().pos, ")");

                            if state.is_some() {}

                            if state.is_none() {
                                state = stack.pop().unwrap();

                                stack.push(state.clone());

                                state = literal(
                                    memo,
                                    key.clone(),
                                    input,
                                    state.clone().unwrap().pos,
                                    ":",
                                );

                                if state.is_some() {}

                                if state.is_none() {
                                    state = stack.pop().unwrap();

                                    stack.push(state.clone());

                                    state = literal(
                                        memo,
                                        key.clone(),
                                        input,
                                        state.clone().unwrap().pos,
                                        "->",
                                    );

                                    if state.is_some() {}

                                    if state.is_none() {
                                        state = stack.pop().unwrap();

                                        stack.push(state.clone());

                                        state = literal(
                                            memo,
                                            key.clone(),
                                            input,
                                            state.clone().unwrap().pos,
                                            "*",
                                        );

                                        if state.is_some() {}

                                        if state.is_none() {
                                            state = stack.pop().unwrap();

                                            state = literal(
                                                memo,
                                                key.clone(),
                                                input,
                                                state.clone().unwrap().pos,
                                                "?",
                                            );

                                            if state.is_some() {}
                                        } else {
                                            stack.pop().unwrap();
                                        }
                                    } else {
                                        stack.pop().unwrap();
                                    }
                                } else {
                                    stack.pop().unwrap();
                                }
                            } else {
                                stack.pop().unwrap();
                            }
                        } else {
                            stack.pop().unwrap();
                        }
                    } else {
                        stack.pop().unwrap();
                    }
                } else {
                    stack.pop().unwrap();
                }
            } else {
                stack.pop().unwrap();
            }
        } else {
            stack.pop().unwrap();
        }
    } else {
        stack.pop().unwrap();
    }

    memo.insert(key, state.clone());
    state
}

fn parse_name<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "name", input, pos);
    let key = format!("name::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = parse_namechar(memo, key.clone(), input, state.clone().unwrap().pos);

    let mut c = String::new();
    if state.is_some() {
        c = state.clone().unwrap().value;
    }

    if state.is_some() {
        state = parse_name(memo, key.clone(), input, state.clone().unwrap().pos);

        let mut n = String::new();
        if state.is_some() {
            n = state.clone().unwrap().value;
        }

        if state.is_some() {
            state = Some(State {
                value: (c + &n),
                pos: state.clone().unwrap().pos,
            });
        }
    }

    if state.is_none() {
        state = stack.pop().unwrap();

        state = parse_namechar(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {}
    } else {
        stack.pop().unwrap();
    }

    memo.insert(key, state.clone());
    state
}

fn parse_namechar<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "namechar", input, pos);
    let key = format!("namechar::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = parse_meta(memo, key.clone(), input, state.clone().unwrap().pos);

    if state.is_some() {
        stack.pop().unwrap();
        state = None;
    } else {
        state = stack.pop().unwrap();
    }

    if state.is_some() {
        stack.push(state.clone());

        state = parse_sp(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            stack.pop().unwrap();
            state = None;
        } else {
            state = stack.pop().unwrap();
        }

        if state.is_some() {
            state = parse_char(memo, key.clone(), input, state.clone().unwrap().pos);

            if state.is_some() {}
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_term<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "term", input, pos);
    let key = format!("term::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = parse_labeled(memo, key.clone(), input, state.clone().unwrap().pos);

    if state.is_some() {}

    if state.is_none() {
        state = stack.pop().unwrap();

        stack.push(state.clone());

        state = parse_star(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {}

        if state.is_none() {
            state = stack.pop().unwrap();

            stack.push(state.clone());

            state = parse_question(memo, key.clone(), input, state.clone().unwrap().pos);

            if state.is_some() {}

            if state.is_none() {
                state = stack.pop().unwrap();

                stack.push(state.clone());

                state = parse_nonterminal(memo, key.clone(), input, state.clone().unwrap().pos);

                if state.is_some() {}

                if state.is_none() {
                    state = stack.pop().unwrap();

                    stack.push(state.clone());

                    state = parse_string(memo, key.clone(), input, state.clone().unwrap().pos);

                    if state.is_some() {}

                    if state.is_none() {
                        state = stack.pop().unwrap();

                        stack.push(state.clone());

                        state =
                            parse_negation(memo, key.clone(), input, state.clone().unwrap().pos);

                        if state.is_some() {}

                        if state.is_none() {
                            state = stack.pop().unwrap();

                            state = parse_parenthesized(
                                memo,
                                key.clone(),
                                input,
                                state.clone().unwrap().pos,
                            );

                            if state.is_some() {}
                        } else {
                            stack.pop().unwrap();
                        }
                    } else {
                        stack.pop().unwrap();
                    }
                } else {
                    stack.pop().unwrap();
                }
            } else {
                stack.pop().unwrap();
            }
        } else {
            stack.pop().unwrap();
        }
    } else {
        stack.pop().unwrap();
    }

    memo.insert(key, state.clone());
    state
}

fn parse_nonterminal<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "nonterminal", input, pos);
    let key = format!("nonterminal::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = parse_name(memo, key.clone(), input, state.clone().unwrap().pos);

    let mut n = String::new();
    if state.is_some() {
        n = state.clone().unwrap().value;
    }

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            state = Some(State {
                value: (format!(
                    "
                            state = parse_{n}(memo, key.clone(), input, state.clone().unwrap().pos);
                        "
                )),
                pos: state.clone().unwrap().pos,
            });
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_labeled<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "labeled", input, pos);
    let key = format!("labeled::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = parse_name(memo, key.clone(), input, state.clone().unwrap().pos);

    let mut label = String::new();
    if state.is_some() {
        label = state.clone().unwrap().value;
    }

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            state = literal(memo, key.clone(), input, state.clone().unwrap().pos, ":");

            if state.is_some() {
                state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

                if state.is_some() {
                    state = parse_term(memo, key.clone(), input, state.clone().unwrap().pos);

                    let mut value = String::new();
                    if state.is_some() {
                        value = state.clone().unwrap().value;
                    }

                    if state.is_some() {
                        state = Some(State {
                            value: (format!(
                                "
                        {value}
                        let mut {label} = String::new();
                        if state.is_some() {{
                            {label} = state.clone().unwrap().value;
                        }}
                    "
                            )),
                            pos: state.clone().unwrap().pos,
                        });
                    }
                }
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_star<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "star", input, pos);
    let key = format!("star::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = parse_string(memo, key.clone(), input, state.clone().unwrap().pos);

    if state.is_some() {}

    if state.is_none() {
        state = stack.pop().unwrap();

        stack.push(state.clone());

        state = parse_nonterminal(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {}

        if state.is_none() {
            state = stack.pop().unwrap();

            state = parse_parenthesized(memo, key.clone(), input, state.clone().unwrap().pos);

            if state.is_some() {}
        } else {
            stack.pop().unwrap();
        }
    } else {
        stack.pop().unwrap();
    }

    let mut value = String::new();
    if state.is_some() {
        value = state.clone().unwrap().value;
    }

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "*");

            if state.is_some() {
                state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

                if state.is_some() {
                    state = Some(State {
                        value: (format!(
                            "
        stack.push(state.clone());
        let mut is_first = true;
        loop {{
            {value}
            if state.is_some() {{
                let last_state = stack.pop().unwrap();
                let mut cur_state = state.clone().unwrap();
                if !is_first && last_state.is_some() {{
                    cur_state.value = last_state.unwrap().value + &cur_state.value;
                }}
                stack.push(Some(cur_state));
            }} else {{
                break;
            }}
            is_first = false;
        }}
        state = stack.pop().unwrap();
        "
                        )),
                        pos: state.clone().unwrap().pos,
                    });
                }
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_question<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "question", input, pos);
    let key = format!("question::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = parse_string(memo, key.clone(), input, state.clone().unwrap().pos);

    if state.is_some() {}

    if state.is_none() {
        state = stack.pop().unwrap();

        stack.push(state.clone());

        state = parse_nonterminal(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {}

        if state.is_none() {
            state = stack.pop().unwrap();

            state = parse_parenthesized(memo, key.clone(), input, state.clone().unwrap().pos);

            if state.is_some() {}
        } else {
            stack.pop().unwrap();
        }
    } else {
        stack.pop().unwrap();
    }

    let mut value = String::new();
    if state.is_some() {
        value = state.clone().unwrap().value;
    }

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "?");

            if state.is_some() {
                state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

                if state.is_some() {
                    state = Some(State {
                        value: (format!(
                            "
        stack.push(state.clone());
        {value}
        if state.is_some() {{
            stack.pop();
            stack.push(state.clone());
        }}
        state = stack.pop().unwrap();
        "
                        )),
                        pos: state.clone().unwrap().pos,
                    });
                }
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_sequence<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "sequence", input, pos);
    let key = format!("sequence::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = parse_term(memo, key.clone(), input, state.clone().unwrap().pos);

    let mut foo = String::new();
    if state.is_some() {
        foo = state.clone().unwrap().value;
    }

    if state.is_some() {
        state = parse_sequence(memo, key.clone(), input, state.clone().unwrap().pos);

        let mut bar = String::new();
        if state.is_some() {
            bar = state.clone().unwrap().value;
        }

        if state.is_some() {
            state = Some(State {
                value: (format!(
                    "
                            {foo}
                            if state.is_some() {{
                                {bar}
                            }}
                        "
                )),
                pos: state.clone().unwrap().pos,
            });
        }
    }

    if state.is_none() {
        state = stack.pop().unwrap();

        stack.push(state.clone());

        state = parse_result_expression(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {}

        if state.is_none() {
            state = stack.pop().unwrap();

            state = Some(State {
                value: ("".to_string()),
                pos: state.clone().unwrap().pos,
            });
        } else {
            stack.pop().unwrap();
        }
    } else {
        stack.pop().unwrap();
    }

    memo.insert(key, state.clone());
    state
}

fn parse_string<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "string", input, pos);
    let key = format!("string::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "\'");

    if state.is_some() {
        state = parse_stringcontents(memo, key.clone(), input, state.clone().unwrap().pos);

        let mut s = String::new();
        if state.is_some() {
            s = state.clone().unwrap().value;
        }

        if state.is_some() {
            state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "\'");

            if state.is_some() {
                state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

                if state.is_some() {
                    state = Some(State {
                        value: (format!(
                            "
                    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, \"{s}\");
                "
                        )),
                        pos: state.clone().unwrap().pos,
                    });
                }
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_stringcontents<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "stringcontents", input, pos);
    let key = format!("stringcontents::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    stack.push(state.clone());

    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "\\");

    if state.is_some() {
        stack.pop().unwrap();
        state = None;
    } else {
        state = stack.pop().unwrap();
    }

    if state.is_some() {
        stack.push(state.clone());

        state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "\'");

        if state.is_some() {
            stack.pop().unwrap();
            state = None;
        } else {
            state = stack.pop().unwrap();
        }

        if state.is_some() {
            state = parse_char(memo, key.clone(), input, state.clone().unwrap().pos);

            let mut c = String::new();
            if state.is_some() {
                c = state.clone().unwrap().value;
            }

            if state.is_some() {
                state = parse_stringcontents(memo, key.clone(), input, state.clone().unwrap().pos);

                let mut s = String::new();
                if state.is_some() {
                    s = state.clone().unwrap().value;
                }

                if state.is_some() {
                    state = Some(State {
                        value: (c + &s),
                        pos: state.clone().unwrap().pos,
                    });
                }
            }
        }
    }

    if state.is_none() {
        state = stack.pop().unwrap();

        stack.push(state.clone());

        state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "\\");

        let mut b = String::new();
        if state.is_some() {
            b = state.clone().unwrap().value;
        }

        if state.is_some() {
            state = parse_char(memo, key.clone(), input, state.clone().unwrap().pos);

            let mut c = String::new();
            if state.is_some() {
                c = state.clone().unwrap().value;
            }

            if state.is_some() {
                state = parse_stringcontents(memo, key.clone(), input, state.clone().unwrap().pos);

                let mut s = String::new();
                if state.is_some() {
                    s = state.clone().unwrap().value;
                }

                if state.is_some() {
                    state = Some(State {
                        value: (b + &c + &s),
                        pos: state.clone().unwrap().pos,
                    });
                }
            }
        }

        if state.is_none() {
            state = stack.pop().unwrap();

            state = Some(State {
                value: ("".to_string()),
                pos: state.clone().unwrap().pos,
            });
        } else {
            stack.pop().unwrap();
        }
    } else {
        stack.pop().unwrap();
    }

    memo.insert(key, state.clone());
    state
}

fn parse_choice<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "choice", input, pos);
    let key = format!("choice::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    state = parse_sequence(memo, key.clone(), input, state.clone().unwrap().pos);

    let mut a = String::new();
    if state.is_some() {
        a = state.clone().unwrap().value;
    }

    if state.is_some() {
        state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "/");

        if state.is_some() {
            state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

            if state.is_some() {
                state = parse_choice(memo, key.clone(), input, state.clone().unwrap().pos);

                let mut b = String::new();
                if state.is_some() {
                    b = state.clone().unwrap().value;
                }

                if state.is_some() {
                    state = Some(State {
                        value: (format!(
                            "
                        stack.push(state.clone());
                        {a}
                        if state.is_none() {{
                            state = stack.pop().unwrap();
                            {b}
                        }} else {{
                            stack.pop().unwrap();
                        }}
                    "
                        )),
                        pos: state.clone().unwrap().pos,
                    });
                }
            }
        }
    }

    if state.is_none() {
        state = stack.pop().unwrap();

        state = parse_sequence(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {}
    } else {
        stack.pop().unwrap();
    }

    memo.insert(key, state.clone());
    state
}

fn parse_negation<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "negation", input, pos);
    let key = format!("negation::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "!");

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            state = parse_term(memo, key.clone(), input, state.clone().unwrap().pos);

            let mut t = String::new();
            if state.is_some() {
                t = state.clone().unwrap().value;
            }

            if state.is_some() {
                state = Some(State {
                    value: (format!(
                        "
                        stack.push(state.clone());
                        {t}
                        if state.is_some() {{
                            stack.pop().unwrap();
                            state = None;
                        }} else {{
                            state = stack.pop().unwrap();
                        }}
                    "
                    )),
                    pos: state.clone().unwrap().pos,
                });
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_result_expression<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "result_expression", input, pos);
    let key = format!("result_expression::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "->");

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            state = parse_expr(memo, key.clone(), input, state.clone().unwrap().pos);

            let mut result = String::new();
            if state.is_some() {
                result = state.clone().unwrap().value;
            }

            if state.is_some() {
                state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

                if state.is_some() {
                    state = Some(State {
                        value: (format!(
                            "
                                state = Some(State {{
                                    value: {result},
                                    pos: state.clone().unwrap().pos,
                                }});
                                "
                        )),
                        pos: state.clone().unwrap().pos,
                    });
                }
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_expr<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "expr", input, pos);
    let key = format!("expr::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "(");

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            state = parse_exprcontents(memo, key.clone(), input, state.clone().unwrap().pos);

            let mut e = String::new();
            if state.is_some() {
                e = state.clone().unwrap().value;
            }

            if state.is_some() {
                state = literal(memo, key.clone(), input, state.clone().unwrap().pos, ")");

                if state.is_some() {
                    state = Some(State {
                        value: ("(".to_string() + &e + ")"),
                        pos: state.clone().unwrap().pos,
                    });
                }
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

fn parse_exprcontents<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "exprcontents", input, pos);
    let key = format!("exprcontents::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    stack.push(state.clone());

    stack.push(state.clone());

    stack.push(state.clone());

    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "(");

    if state.is_some() {
        stack.pop().unwrap();
        state = None;
    } else {
        state = stack.pop().unwrap();
    }

    if state.is_some() {
        stack.push(state.clone());

        state = literal(memo, key.clone(), input, state.clone().unwrap().pos, ")");

        if state.is_some() {
            stack.pop().unwrap();
            state = None;
        } else {
            state = stack.pop().unwrap();
        }

        if state.is_some() {
            state = parse_char(memo, key.clone(), input, state.clone().unwrap().pos);

            if state.is_some() {}
        }
    }

    if state.is_none() {
        state = stack.pop().unwrap();

        state = parse_expr(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {}
    } else {
        stack.pop().unwrap();
    }

    let mut c = String::new();
    if state.is_some() {
        c = state.clone().unwrap().value;
    }

    if state.is_some() {
        state = parse_exprcontents(memo, key.clone(), input, state.clone().unwrap().pos);

        let mut e = String::new();
        if state.is_some() {
            e = state.clone().unwrap().value;
        }

        if state.is_some() {
            state = Some(State {
                value: (c + &e),
                pos: state.clone().unwrap().pos,
            });
        }
    }

    if state.is_none() {
        state = stack.pop().unwrap();

        state = Some(State {
            value: ("".to_string()),
            pos: state.clone().unwrap().pos,
        });
    } else {
        stack.pop().unwrap();
    }

    memo.insert(key, state.clone());
    state
}

fn parse_parenthesized<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    add_operation(memo, parent, "parenthesized", input, pos);
    let key = format!("parenthesized::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    state = Some(State {
        value: "".to_string(),
        pos: pos,
    });
    let mut stack: Vec<Option<State<String>>> = Vec::new();

    state = literal(memo, key.clone(), input, state.clone().unwrap().pos, "(");

    if state.is_some() {
        state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

        if state.is_some() {
            state = parse_choice(memo, key.clone(), input, state.clone().unwrap().pos);

            let mut body = String::new();
            if state.is_some() {
                body = state.clone().unwrap().value;
            }

            if state.is_some() {
                state = literal(memo, key.clone(), input, state.clone().unwrap().pos, ")");

                if state.is_some() {
                    state = parse__(memo, key.clone(), input, state.clone().unwrap().pos);

                    if state.is_some() {
                        state = Some(State {
                            value: (body),
                            pos: state.clone().unwrap().pos,
                        });
                    }
                }
            }
        }
    }

    memo.insert(key, state.clone());
    state
}

pub fn parse_peg(input: &str) -> String {
    let mut memo = Memo::new();
    let ans = parse_start(&mut memo, "".to_string(), input, 0);
    if ans.is_none() {
        return "".to_string();
    }
    memo.print_operations(input);
    ans.unwrap().value
}
