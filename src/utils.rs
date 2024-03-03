use std::{collections::HashMap, io::Write};

pub fn add_operation<'a, 'b, 'c>(
    memo: &'a mut Memo<String>,
    parent: String,
    name: &'c str,
    input: &'b str,
    pos: usize,
) {
    let (row, col) = transform_index_2_rowcol(input, pos);
    memo.add_operation(parent, name.to_string(), pos, row, col);
}

pub fn transform_index_2_rowcol(input: &str, index: usize) -> (usize, usize) {
    let mut row: usize = 1;
    let mut col: usize = 1;
    input[..index].chars().for_each(|ch| match ch {
        '\n' => {
            row += 1;
            col = 1;
        }
        '\r' => (),
        _ => col += 1,
    });
    (row, col)
}

#[derive(Clone, Debug)]
pub struct State<T> {
    pub value: T,
    pub pos: usize,
}

pub struct Operation {
    pub parent: String,
    pub name: String,
    pub position: usize,
    pub row: usize,
    pub col: usize,
}

pub struct Memo<T: Clone> {
    store: HashMap<String, Option<State<T>>>,
    operations: Vec<Operation>,
}

impl<T: Clone> Memo<T> {
    pub fn new() -> Self {
        Memo {
            store: HashMap::new(),
            operations: Vec::new(),
        }
    }

    pub fn get(&self, key: String) -> (Option<State<T>>, bool) {
        let state = self.store.get(&key);
        if state.is_some() {
            return (state.unwrap().clone(), true);
        }
        (None, false)
    }

    pub fn insert(&mut self, key: String, state: Option<State<T>>) {
        self.store.insert(key, state);
    }

    pub fn add_operation(
        &mut self,
        parent: String,
        name: String,
        position: usize,
        row: usize,
        col: usize,
    ) {
        self.operations.push(Operation {
            parent: parent,
            name: name,
            position: position,
            row: row,
            col: col,
        })
    }

    pub fn print_operations(&self, input: &str) {
        use std::fs::File;
        let mut ans = String::new();
        for operation in self.operations.as_slice() {
            let key = format!("{}::{}", operation.name, operation.position);
            let (result, existed) = self.get(key);
            let result_str = if existed {
                if result.is_some() {
                    format!(
                        "ACCEPTED\r
matched: {}
                    ",
                        input[operation.position..result.unwrap().pos].to_string()
                    )
                } else {
                    "REJECTED".to_string()
                }
            } else {
                "UNKNOWN".to_string()
            };
            ans += &format!("parse_{}, parent: {}\r\n", operation.name, operation.parent);
            ans += &format!(
                "  position: {}, row: {}, col: {}, result: {}\r\n",
                operation.position, operation.row, operation.col, result_str
            );
        }
        ans += &format!("total: {}", self.operations.len());
        let mut file = File::create("./operations.txt").unwrap();
        file.write_all(ans.as_bytes()).unwrap();
        print!("{}", ans)
    }
}

pub fn literal<'a, 'b>(
    memo: &'a mut Memo<String>,
    parent: String,
    input: &'b str,
    pos: usize,
    s: &str,
) -> Option<State<String>> {
    let name = &format!("literal: {s}");
    let key = format!("{name}::{pos}");
    let (mut state, existed) = memo.get(key.clone());
    if existed {
        return state;
    }
    add_operation(memo, parent, name, input, pos);
    if input[pos..].to_string().starts_with(s) {
        state = Some(State {
            pos: pos + s.len(),
            value: s.to_string(),
        });
    } else {
        state = None;
    }
    memo.insert(key, state.clone());
    state
}

pub fn parse_char<'a, 'b>(
    memo: &'a Memo<String>,
    _: String,
    input: &'b str,
    pos: usize,
) -> Option<State<String>> {
    if pos >= input.len() {
        return None;
    }
    Some(State {
        value: input[pos..pos + 1].to_string(),
        pos: pos + 1,
    })
}
