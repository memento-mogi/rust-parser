#[derive(PartialEq, Debug, Clone)]
pub enum OperatorType {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    Lt,
    Ltoeq,
    Gt,
    Gtoeq,
    // Add more operators as needed
}

#[derive(PartialEq, Debug, Clone)]
pub enum BooleanType {
    True,
    False,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LexerState {
    Initial,
    Numbers,
    Alphabets
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType{
    Identifier(String),
    Number(usize),
    Boolean(BooleanType),
    Operator(OperatorType),
    Let,
    Assignment,
    In,
    EOF
}

pub struct Lexer<'a> {
    input: &'a str,
    state: LexerState,
    start: usize,
    current: usize,
    current_token: Option<TokenType>,
    tokens: Vec<TokenType>,
}
    
impl<'a> Lexer<'a> {
    pub fn new (input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            state: LexerState::Initial,
            start: 0,
            current: 0,
            current_token: None,
            tokens: Vec::new(),
        }
    }

    fn push_current_token(&mut self) {
        if let Some(token) = &self.current_token {
            self.tokens.push(token.clone());
            self.current_token = None;
        }
        self.start = self.current - 1;
    }

    fn push_token(&mut self, token: TokenType) {
        self.tokens.push(token);
        self.current_token = None;
        self.start = self.current;
    }

    fn lt_ahead(&mut self) -> TokenType {
        if let Some(next_char) = self.input.chars().nth(self.current) {
            if next_char == '=' {
                self.current += 1;
                TokenType::Operator(OperatorType::Ltoeq)
            } else {
                TokenType::Operator(OperatorType::Lt)
            }
        } else {
            TokenType::Operator(OperatorType::Lt)
        }
    }

    fn gt_ahead(&mut self) -> TokenType {
        if let Some(next_char) = self.input.chars().nth(self.current) {
            if next_char == '=' {
                self.current += 1;
                TokenType::Operator(OperatorType::Gtoeq)
            } else {
                TokenType::Operator(OperatorType::Gt)
            }
        } else {
            TokenType::Operator(OperatorType::Gt)
        }
    }

    fn eq_ahead(&mut self) -> TokenType {
        if let Some(next_char) = self.input.chars().nth(self.current) {
            if next_char == '=' {
                self.current += 1;
                TokenType::Operator(OperatorType::Equal)
            } else {
                TokenType::Assignment
            }
        } else {
            TokenType::Assignment
        }
    }

    fn lex_char(&mut self) {
        self.current = self.current + 1;
        let symbol_collection = ['+', '-', '*', '/', '<', '>', '='];
        match self.input.chars().nth(self.current - 1) {
            Some(c) if c.is_whitespace() || symbol_collection.contains(&c) => {
                self.push_current_token();
                if !c.is_whitespace() {
                    let token = match c {
                        '+' => TokenType::Operator(OperatorType::Plus),
                        '-' => TokenType::Operator(OperatorType::Minus),
                        '*' => TokenType::Operator(OperatorType::Asterisk),
                        '/' => TokenType::Operator(OperatorType::Slash),
                        '<' => self.lt_ahead(),
                        '>' => self.gt_ahead(),
                        '=' => self.eq_ahead(),
                        _ => unreachable!(),
                    };
                    self.push_token(token);
                }
                self.start = self.current;
                self.state = LexerState::Initial;
            }
            Some(c) => {
                if c.is_numeric() && (self.state == LexerState::Initial || self.state == LexerState::Numbers) {
                    let current_number = &self.input[self.start..self.current];
                    let number_value: usize = current_number.parse().unwrap();
                    self.current_token = Some(TokenType::Number(number_value));
                    self.state = LexerState::Numbers;
                } else {
                    if let Some(TokenType::Number(_)) = &self.current_token {
                        self.push_current_token();
                    }
                    let current_str = &self.input[self.start..self.current];
                    let keyword_list = ["true", "false", "let", "in"];
                    if keyword_list.contains(&current_str) {
                        let keyword_token = match current_str {
                            "true" => TokenType::Boolean(BooleanType::True),
                            "false" => TokenType::Boolean(BooleanType::False),
                            "let" => TokenType::Let,
                            "in" => TokenType::In,
                            _ => unreachable!()
                        };
                        self.current_token = Some(keyword_token);
                    } else {
                        self.current_token = Some(TokenType::Identifier(current_str.to_string()));
                    };
                    self.state = LexerState::Alphabets;
                }
            }
            None => {panic!("Unexpected end of input");}
        }
    }

    pub fn exec (&mut self) -> Vec<TokenType> {
        while self.current < self.input.len() {
            self.lex_char();
        }
        self.push_current_token();
        let mut tokens = self.tokens.clone();
        tokens.push(TokenType::EOF);
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexer_test() {
        let mut lexer = Lexer::new("let num = 3 in num * num");
        let tokens = lexer.exec();
        assert_eq!(tokens, vec![
            TokenType::Operator(OperatorType::Equal),
            TokenType::Assignment,
            TokenType::Identifier("a".to_string()),
            TokenType::Operator(OperatorType::Ltoeq),
            TokenType::Boolean(BooleanType::True),
            TokenType::Operator(OperatorType::Gt),
            TokenType::EOF,
        ]);
    }
}



    // fn lex_char(&mut self) {
    //     self.current = self.current + 1;
    //     // let symbol_collection = ['+', '-', '*', '/'];
    //     match self.input.chars().nth(self.current - 1) {
    //         Some(c) if c.is_whitespace()  => {
    //             self.push_current_token();
    //         }
    //         Some(c) => {
    //             let current_id = &self.input[self.start..self.current];
    //             // self.lex_symbols();
    //             let keyword_list = ["+", "-", "*", "/", "=", "<", ">", "true", "false"];
    //             if keyword_list.contains(&current_id) {
    //                 let keyword_token = match current_id {
    //                     "+"  => TokenType::Operator(OperatorType::Plus),
    //                     "-" => TokenType::Operator(OperatorType::Minus),
    //                     "*" => TokenType::Operator(OperatorType::Asterisk),
    //                     "/" => TokenType::Operator(OperatorType::Slash),
    //                     "<" => self.lt_ahead(),
    //                     ">" => self.gt_ahead(),
    //                     "=" => self.eq_ahead(),
    //                     "true" => TokenType::Boolean(BooleanType::True),
    //                     "false" => TokenType::Boolean(BooleanType::False),
    //                     _ => unreachable!()
    //                 };
    //                 self.push_token(keyword_token);
    //             } else if {
                    
    //             } else {
    //                 self.current_token = Some(TokenType::Identifier(current_id.to_string()));
    //             };
    //         }
    //         None => {panic!("Unexpected end of input");}
    //     }
    // }
