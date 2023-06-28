# Usage

```
$ cargo run
> 1 + 1
2
> 2 * 3 / (4 - 5)
-6
> 13+21
34
```

# Operators

| priority | symbol | associativity |
| -------- | ------ | ------------- |
| 3 (high) | +, -   | (unary ope)   |
| 2        | *, /   | left          |
| 1 (low)  | +, -   | left          |

# EBNF


```plantuml
@startebnf
EXPR = EXPR3;

EXPR3 = EXPR3, ("+" | "-"), EXPR2 | EXPR2;
EXPR2 = EXRP2, ("*" | "/"), EXRP1 | EXPR1;
EXPR1 = ("+" | "-"), ATOM | ATOM;
ATOM = UNUMBER | "(", EXPR3, ")";
UNUMBER = DIGIT | {DIGIT};
DIGIT = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
@endebnf
```

# grand design

```plantuml
@startuml
state "入力" as Input
state "トークン列" as Tokens
state "内部表現" as Expr
state "インタプリタ" as Interpreter
state "RPNへコンパイル" as Compile
state "出力" as Output

Input --> Tokens : __字句解析
Tokens --> Expr : __構文解析
Expr --> Interpreter
Expr --> Compile
Compile --> Output
@enduml
```

# Lexer

```
                  [0-9]
                 +------+
        [0-9]    ↓      |  [^0-9]
      +-------> [] ----- -----------> (Number)
      |
      |   +
[*] --|-------> (Plus)
      |
      |   -
      +-------> (Minus)
      |
      |   *
      +-------> (Asterisk)
      |
      |   /
      +-------> (Slash)
      |
      |   (
      +-------> (LParen)
      |
      |   )
      +-------> (RParen)
      
      
```