use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
enum BuiltIn {
    Add,
    Subtract,
    Multiply,
    Divide,
    Dup,
    Drop,
    Swap,
    Over,
    Define,
    EndDefine,
}

#[derive(Copy, Clone, Debug)]
struct WordId(usize);

#[derive(Copy, Clone, Debug)]
enum Command {
    Arg(Value),
    Word(WordId),
    BuiltInWord(BuiltIn),
}

pub struct Forth {
    stack: Stack,
    dictionary: Dictionary,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            dictionary: Dictionary::new(),
        }
    }
    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }
    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut tokens = input.split_whitespace();
        let (stack, dictionary) = (&mut self.stack, &mut self.dictionary);
        while let Some(token) = tokens.next() {
            let token = token.to_ascii_lowercase();
            match dictionary.lookup(token.as_str()) {
                Some(commands) => Self::perform_sub_program(commands, stack, dictionary),
                None => match token.as_str() {
                    "+" => Self::perform_add(stack),
                    "-" => Self::perform_subtract(stack),
                    "*" => Self::perform_multiply(stack),
                    "/" => Self::perform_divide(stack),
                    "dup" => Self::perform_dup(stack),
                    "drop" => Self::perform_drop(stack),
                    "swap" => Self::perform_swap(stack),
                    "over" => Self::perform_over(stack),
                    ":" => Self::perform_define_sub_program(&mut tokens, dictionary),
                    ";" => Err(Error::InvalidWord),
                    token => Self::perform_push_token_as_number(token, stack),
                },
            }?
        }
        Ok(())
    }
    fn perform_add(stack: &mut Stack) -> ForthResult {
        let (a, b) = (stack.pop()?, stack.pop()?);
        stack.push(b + a);
        Ok(())
    }
    fn perform_subtract(stack: &mut Stack) -> ForthResult {
        let (a, b) = (stack.pop()?, stack.pop()?);
        stack.push(b - a);
        Ok(())
    }
    fn perform_multiply(stack: &mut Stack) -> ForthResult {
        let (a, b) = (stack.pop()?, stack.pop()?);
        stack.push(b * a);
        Ok(())
    }
    fn perform_divide(stack: &mut Stack) -> ForthResult {
        let (a, b) = (stack.pop()?, stack.pop()?);
        if a == 0 {
            return Err(Error::DivisionByZero);
        }
        stack.push(b / a);
        Ok(())
    }
    fn perform_dup(stack: &mut Stack) -> ForthResult {
        stack.dup()?;
        Ok(())
    }
    fn perform_drop(stack: &mut Stack) -> ForthResult {
        stack.drop()?;
        Ok(())
    }
    fn perform_swap(stack: &mut Stack) -> ForthResult {
        stack.swap()?;
        Ok(())
    }
    fn perform_over(stack: &mut Stack) -> ForthResult {
        stack.over()?;
        Ok(())
    }
    fn perform_define_sub_program<'a>(
        tokens: &mut impl Iterator<Item = &'a str>,
        dictionary: &mut Dictionary,
    ) -> ForthResult {
        let mut commands = Vec::new();
        let word = match tokens.next() {
            Some(":") | Some(";") | None => return Err(Error::InvalidWord),
            Some(word) if Self::is_valid_number(word) => return Err(Error::InvalidWord),
            Some(word) => word.to_ascii_lowercase(),
        };
        let mut terminated_properly = false;
        for token in tokens {
            let token = token.to_ascii_lowercase();
            let command = match dictionary.lookup_id(token.as_str()) {
                Some(word_id) => Command::Word(word_id),
                None => match token.as_str() {
                    "+" => Command::BuiltInWord(BuiltIn::Add),
                    "-" => Command::BuiltInWord(BuiltIn::Subtract),
                    "*" => Command::BuiltInWord(BuiltIn::Multiply),
                    "/" => Command::BuiltInWord(BuiltIn::Divide),
                    "dup" => Command::BuiltInWord(BuiltIn::Dup),
                    "drop" => Command::BuiltInWord(BuiltIn::Drop),
                    "swap" => Command::BuiltInWord(BuiltIn::Swap),
                    "over" => Command::BuiltInWord(BuiltIn::Over),
                    ";" => {
                        terminated_properly = true;
                        break;
                    }
                    ":" => return Err(Error::InvalidWord),
                    token => {
                        if let Some(number) = Self::parse_valid_number(token) {
                            Command::Arg(number)
                        } else {
                            return Err(Error::UnknownWord);
                        }
                    }
                },
            };
            commands.push(command);
        }
        if commands.is_empty() || !terminated_properly {
            return Err(Error::InvalidWord);
        } else {
            Self::add_word_to_dictionary(word, commands, dictionary);
        }
        Ok(())
    }
    fn perform_push_token_as_number(token: &str, stack: &mut Stack) -> ForthResult {
        let num = str::parse::<i32>(token).map_err(|_| Error::UnknownWord)?;
        Self::perform_push_number(num, stack)
    }
    fn perform_push_number(num: i32, stack: &mut Stack) -> ForthResult {
        stack.push(num);
        Ok(())
    }
    fn perform_sub_program(
        commands: &[Command],
        stack: &mut Stack,
        dictionary: &Dictionary,
    ) -> ForthResult {
        for command in commands.iter().map(|c| *c) {
            match command {
                Command::Word(word_id) => {
                    let commands = dictionary.definition(word_id);
                    Self::perform_sub_program(commands, stack, dictionary)
                }
                Command::Arg(number) => Self::perform_push_number(number, stack),
                Command::BuiltInWord(BuiltIn::Add) => Self::perform_add(stack),
                Command::BuiltInWord(BuiltIn::Subtract) => Self::perform_subtract(stack),
                Command::BuiltInWord(BuiltIn::Multiply) => Self::perform_multiply(stack),
                Command::BuiltInWord(BuiltIn::Divide) => Self::perform_divide(stack),
                Command::BuiltInWord(BuiltIn::Dup) => Self::perform_dup(stack),
                Command::BuiltInWord(BuiltIn::Drop) => Self::perform_drop(stack),
                Command::BuiltInWord(BuiltIn::Swap) => Self::perform_swap(stack),
                Command::BuiltInWord(BuiltIn::Over) => Self::perform_over(stack),
                command => panic!("Should not occcur - invalid command: {:?}", command),
            }?
        }
        Ok(())
    }
    fn parse_valid_number(token: &str) -> Option<Value> {
        match str::parse::<i32>(token) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
    fn is_valid_number(token: &str) -> bool {
        Self::parse_valid_number(token).is_some()
    }
    fn add_word_to_dictionary(word: String, commands: Vec<Command>, dictionary: &mut Dictionary) {
        let (words, definitions) = (&mut dictionary.words, &mut dictionary.definitions);
        words
            .entry(word)
            .and_modify(|word_id| {
                *word_id = Self::define_new_word_in_dictionary(&commands, definitions)
            })
            .or_insert_with(|| Self::define_new_word_in_dictionary(&commands, definitions));
    }
    fn define_new_word_in_dictionary(
        commands: &[Command],
        definitions: &mut Vec<Vec<Command>>,
    ) -> usize {
        definitions.push(commands.to_vec());
        definitions.len() - 1
    }
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}

struct Stack {
    values: Vec<Value>,
}

impl Stack {
    fn new() -> Self {
        Self { values: Vec::new() }
    }
    fn as_slice(&self) -> &[Value] {
        &self.values
    }
    fn pop(&mut self) -> Result<Value, Error> {
        match self.values.pop() {
            Some(value) => Ok(value),
            None => Err(Error::StackUnderflow),
        }
    }
    fn push(&mut self, value: Value) {
        self.values.push(value);
    }
    fn peek(&self) -> Option<Value> {
        if !self.values.is_empty() {
            Some(self.values[self.values.len() - 1])
        } else {
            None
        }
    }
    fn dup(&mut self) -> Result<(), Error> {
        match self.peek() {
            Some(value) => {
                self.push(value);
                Ok(())
            }
            None => Err(Error::StackUnderflow),
        }
    }
    fn drop(&mut self) -> Result<(), Error> {
        self.pop().map_err(|_| Error::StackUnderflow)?;
        Ok(())
    }
    fn swap(&mut self) -> Result<(), Error> {
        let (a, b) = (
            self.pop().map_err(|_| Error::StackUnderflow)?,
            self.pop().map_err(|_| Error::StackUnderflow)?,
        );
        self.push(a);
        self.push(b);
        Ok(())
    }
    fn over(&mut self) -> Result<(), Error> {
        let (a, b) = (
            self.pop().map_err(|_| Error::StackUnderflow)?,
            match self.peek() {
                Some(value) => value,
                None => return Err(Error::StackUnderflow),
            },
        );
        self.push(a);
        self.push(b);
        Ok(())
    }
}

struct Dictionary {
    words: HashMap<String, usize>,
    definitions: Vec<Vec<Command>>,
}

impl Dictionary {
    fn new() -> Self {
        Self {
            words: HashMap::new(),
            definitions: Vec::new(),
        }
    }

    fn lookup(&self, word: &str) -> Option<&[Command]> {
        self.words
            .get(word)
            .map(|word_id| self.definitions[*word_id].as_slice())
    }

    fn lookup_id(&self, word: &str) -> Option<WordId> {
        self.words.get(word).map(|id| WordId(*id))
    }

    fn definition(&self, word_id: WordId) -> &[Command] {
        self.definitions[word_id.0].as_slice()
    }
}
