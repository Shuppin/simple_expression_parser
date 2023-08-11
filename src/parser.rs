use crate::{
    tokeniser::{Tokeniser, TokenKind, Token},
    ast::{Node, self}
};

/// Generates a walkable & executable abstract syntax tree out of an expression.
pub struct Parser {
    tokeniser: Tokeniser,
    current_token: Token
}

impl Parser {
    pub fn new(source: String) -> Self {
        Parser {
            tokeniser: Tokeniser::new(source),
            // This empty token acts as a placeholder until the
            // tokeniser is actually invoked.
            current_token: Token::empty()
        }
    }

    /// Setter function to update the source code which needs to be parsed.
    pub fn set_source(&mut self, source: String) {
        // It's easier just to initialise a new tokensier than
        // to individually reset all of it's attributes.
        self.tokeniser = Tokeniser::new(source);
    }

    /// The parse() function is the entry point for the whole
    /// expression parser.
    /// 
    /// Grammar (modified bnf):
    /// 
    /// ```
    /// <expr> ::= <mult_expr> ((Add | Sub) <mult_expr>)*
    /// 
    /// <mult_expr> ::= <entity> ((Mult | Div) <entity>)*
    /// 
    /// <entity> ::= IntLiteral | FloatLiteral | Sub <entity> | LParen <expr> RParen
    /// ```
    /// 
    /// The grammar is in order of scope, the highest covering the entire syntax,
    /// the lowest covering the most fundamental components of an expression.
    /// 
    pub fn parse(&mut self) -> Result<Box<dyn Node>, String> {
        // Explicitly generate the first token.
        self.current_token = self.tokeniser.next_token()?;
        // `expr` is the highest level variable defined in our grammar,
        // This means it covers every single case the parser is capable
        // of parsing.
        let result = self.expr()?;
        Ok(result)
    }

    /// Generates the next token.
    /// 
    /// This function forces us to explicitly declare what token we
    /// are 'eating' before moving onto the next token.
    /// 
    /// For example, if we want to eat an integer, but we get a bracket instead,
    /// we know there is an error in the expression.
    fn eat(&mut self, expected_token_kind: TokenKind) -> Result<(), String> {
        if self.current_token.kind != expected_token_kind {
            Err(format!("Expected kind {:?}, got kind {:?}", expected_token_kind, self.current_token.kind))
        } else {
            self.current_token = self.tokeniser.next_token()?;
            Ok(())
        }
    }

    /// Represents any fundamental mathematical entity.
    /// 
    /// *Technically excluding a mathematical expression
    /// that contains no brackets but this is a limitation
    /// of existing terminology that defines groups of
    /// mathematical concepts. 
    fn entity(&mut self) -> Result<Box<dyn Node>, String> {
        match self.current_token.kind {
            // Literals, things like '10' or '3.14'
            // Also referred to as constants.
            TokenKind::IntLiteral => {
                let ret = Box::new(ast::IntLiteral {
                    value: self.current_token.value.clone().ok_or(0)
                    .expect("property `value` for a token of kind 
                    `TokenKind::IntLiteral` should not be none")
                });
                self.eat(TokenKind::IntLiteral)?;
                Ok(ret)
            },
            TokenKind::FloatLiteral => {
                let ret = Box::new(ast::FloatLiteral {
                    value: self.current_token.value.clone().ok_or(0)
                    .expect("property `value` for a token of kind 
                    `TokenKind::FloatLiteral` should not be none")
                });
                self.eat(TokenKind::FloatLiteral)?;
                Ok(ret)
            },

            // All unary operations begin with a '-' symbol.
            TokenKind::Sub => {
                self.eat(TokenKind::Sub)?;
                Ok(Box::new(ast::UnaryOp {
                    right: self.entity()?,
                    op: ast::Op::Sub
                }))
            }

            // Brackets aren't an object found on the syntax tree,
            // rather, it changes the structure of the tree to
            // represent the order defined by the brackets.
            TokenKind::LParen => {
                self.eat(TokenKind::LParen)?;
                let expr = self.expr()?;
                self.eat(TokenKind::RParen)?;
                Ok(expr)
            }

            // If we encounter any other type of token, this is unexpected so error.
            _ => {
                Err(format!("Unexpected token: {:?} at pos {:?}", self.current_token, self.tokeniser.char_pos))
            }
        }
    }

    /// Represents any mathematical expression containing two or more terms.
    fn expr(&mut self) -> Result<Box<dyn Node>, String> {
        // Get the left hand side of the expression.
        let mut node = self.mult_expr()?;

        // If the expression contains no relevant operators beyond this point,
        // we just return the entity as it is.

        // Else:
        // While the operator is either a '*' or '/'
        while self.current_token.kind == TokenKind::Add
            || self.current_token.kind == TokenKind::Sub {
                
                // Eat the token and map the
                // tokeniser::TokenKind to the matching ast::Op
                let op = match self.current_token.kind {
                    TokenKind::Add => {
                        self.eat(TokenKind::Add)?;
                        ast::Op::Add
                    },
                    TokenKind::Sub => {
                        self.eat(TokenKind::Sub)?;
                        ast::Op::Sub
                    },
                    _ => unreachable!()
                };

                // Create a binary operation object.
                // As this code loops, `left` will become the BinOp
                // from the previous iteration.
                node = Box::new(ast::BinOp {
                    left: node,
                    right: self.mult_expr()?,
                    op
                })
            };

        Ok(node)

    }

    /// Represents any mathematical expression containing two or
    /// more terms using only the '*' and '/' operators.
    fn mult_expr(&mut self) -> Result<Box<dyn Node>, String> {
        // Get the left hand side of the expression.
        let mut node = self.entity()?;

        // If the expression contains no relevant operators beyond this point,
        // we just return the entity as it is.

        // Else:
        // While the operator is either a '*' or '/'
        while self.current_token.kind == TokenKind::Mult
            || self.current_token.kind == TokenKind::Div {
                
                // Eat the token and map the
                // tokeniser::TokenKind to the matching ast::Op
                let op = match self.current_token.kind {
                    TokenKind::Mult => {
                        self.eat(TokenKind::Mult)?;
                        ast::Op::Mult
                    },
                    TokenKind::Div => {
                        self.eat(TokenKind::Div)?;
                        ast::Op::Div
                    },
                    _ => unreachable!()
                };

                // Create a binary operation object.
                // As this code loops, `left` will become the BinOp
                // from the previous iteration.
                node = Box::new(ast::BinOp {
                    left: node,
                    right: self.entity()?,
                    op
                })
            };

        Ok(node)
    }


}