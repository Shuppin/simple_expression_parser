
/// Represents the different types of tokens found within an expression.
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    IntLiteral,
    FloatLiteral,
    Add,
    Sub,
    Mult,
    Div,
    LParen,
    RParen,
    EOF,
    Empty
}

/// A Token is an individual component of an expression.
/// 
/// For example, a token could be a number or mathematical symbol.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
    pos: usize
}

impl Token {
    pub fn empty() -> Self {
        Self {
            kind: TokenKind::Empty,
            value: None,
            pos: 0
        }
    }
}

/// Splits up an expression into it's fundamental parts, creating a token stream.
pub struct Tokeniser {
    source: String,
    pub char_pos: usize,
}

impl Tokeniser {
    pub fn new(source: String) -> Self {
        Self {
            source,
            char_pos: 0
        }
    }

    /// Retrieves the current char without incrementing char_pos
    fn current_char(&self) -> char {
        match self.source.chars().nth(self.char_pos) {
            Some(c) => c,
            None => '\0'
        }
    }

    /// Retrieves the current char and increments char_pos
    fn next_char(&mut self) -> char {
        self.char_pos += 1;
        match self.source.chars().nth(self.char_pos) {
            Some(c) => {
                c
            },
            None => '\0'
        }
    }

    /// Gets a sequence of consectuive numbers
    fn number_sequence(&mut self) -> String {
        let mut char = self.current_char();
        let mut number_string = String::new();
        while char.is_numeric() {
            number_string.push(char);
            char = self.next_char();
        }
        number_string
    }

    /// Generates the next token in the stream.
    /// 
    /// Errors if invalid character sequence is found.
    pub fn next_token(&mut self) -> Result<Token, String> {

        // Skip whitespace
        while self.current_char().is_whitespace() {
            self.next_char();
        }

        match self.current_char() {

            // End of file
            '\0' => {
                Ok(Token {
                    kind: TokenKind::EOF,
                    value: None,
                    pos: self.char_pos
                })
            }

            // Numbers
            c if c.is_numeric() => {
                let starting_char_pos = self.char_pos;
                let mut number_sequence = self.number_sequence();
                // If the character is a decimal point,
                // we are dealing with a FloatLiteral.
                if self.current_char() == '.' {
                    // Add the decimal point
                    number_sequence.push('.');
                    self.next_char();
                    // Get the decimal portion
                    let decimal_sequence = self.number_sequence();
                    if decimal_sequence.len() == 0 {
                        Err(
                            format!("Unfinished FloatLiteral '{}' at position {}", number_sequence, self.char_pos)
                        )
                    }
                    else {
                        // Add the decimal portion to the string value
                        number_sequence.push_str(&decimal_sequence);
    
                        Ok(Token {
                            kind: TokenKind::FloatLiteral,
                            value: Some(number_sequence),
                            pos: starting_char_pos
                        })
                    }
                }
                // Else just return a normal IntLiteral
                else {
                    Ok(Token {
                        kind: TokenKind::IntLiteral,
                        value: Some(number_sequence),
                        pos: starting_char_pos
                    })
                }
            }

            // Single char tokens
            '+' | '-' | '/' | '*' |
            '(' | ')' 
            => {
                // This syntax may look strange, but it massively reduces
                // code length compared having one match statement for
                // each single char token.
                let token_kind = match self.current_char() {
                    '+' => TokenKind::Add,
                    '-' => TokenKind::Sub,
                    '/' => TokenKind::Div,
                    '*' => TokenKind::Mult,
                    '(' => TokenKind::LParen,
                    ')' => TokenKind::RParen,
                    _ => unreachable!()
                };
                self.next_char();
                Ok(Token {
                    kind: token_kind,
                    value: None,
                    // next_char() increments char_pos so we undo that here.
                    pos: self.char_pos-1
                })
            }

            _ => Err(format!("Unrecognised char '{}' at postion {}", self.current_char(), self.char_pos))
        }
    }
}
