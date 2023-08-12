# Simple Expression Parser

Welcome to my simple expression parser. This parser follows a recursive descent approach, but uses a bottom-up execution style. Its purpose is to analyse simple mathematical expressions that involve numbers, basic arithmetic operators (+,-,/,*), and parentheses. 

Additionally, the parser incorporates a basic order of operations. It begins by evaluating brackets, followed by multiplication and division, and finally addition and subtraction. This order is determined by the parser and is represented through the tree's hierarchy.

The parser's design draws from my previous parser and is loosely influenced by the parser used in the rustc compiler (though, that parser is vastly more complicated).

This project uses cargo and you can run it with `cargo run` or `cargo run --release`.

## Demo
The expression `14*(2+3)` produces the output:

<!--- Not actually C code, but it looks the best with github's syntax highlighting --->
```c
BinOp {
    left: IntLiteral {    
        value: 14
    }
    right: BinOp {        
        left: IntLiteral {
            value: 2      
        }
        right: IntLiteral {
            value: 3
        }
        op: Add
    }
    op: Mult
}

answer = 70
```

It outputs the generated tree followed by the evaluated answer. You can see how elements in brackets changes the hierarchy of the tree.

## Parsing

The parser starts its work by using the `parse()` function found in the `parser::Parser` object. This function creates the initial token and then proceeds to examine each following token. For each token, the parser identifies its type and selects the appropriate function to generate an object that adheres to the `Node` trait. If a suitable function isn't available, it means the provided expression doesn't follow the correct syntax, and in such cases, an error is returned.

The `Node` object can contain multiple child objects that also follow the `Node` trait, forming a structure resembling a tree. This arrangement is referred to as an abstract syntax tree (AST). You can locate these `Node` objects, including the trait definition itself, in the `src/ast.rs` file.

The parser's grammar is outlined as follows:

```ebnf
<expr> ::= <mult_expr> ((`Add` | `Sub`) <mult_expr>)*

<mult_expr> ::= <entity> ((`Mult` | `Div`) <entity>)*

<entity> ::= `IntLiteral` | `FloatLiteral` | `Sub` <entity> | `LParen` <expr> `RParen`
```

> Note: The items enclosed by `` `...` `` represent the different types of tokens used in the program.

For each of these grammar rules, there exists a corresponding function within the parser module. Each function returns an object that adheres to the `Node` trait.

## Evaluation

To evaluate an expression, you use the `evaluate()` function on the root node. This function then calls `evaluate()` on its child nodes as needed. Some nodes trigger evaluations down the hierarchy, while others directly produce a value since they lack child nodes. This process continues until a final value is obtained, which is then returned by the function.

This evaluation approach means that each expression corresponds to only one value. As a result, this method isn't suitable for more complex parsers, such as those involving variables.
