#[derive(Debug, Clone, PartialEq)]
pub enum Syntax {
    // 関数定義
    GlobalFn,    // global fn
    Fn,         // fn
    Arrow,      // ->
    
    // 型
    Type(String),     // uint8, int32など
    
    // 変数宣言と代入
    Identifier(String), // 変数名
    Equals,            // =
    
    // 制御構文
    If,        // if
    Else,      // else
    While,     // while
    For,       // for
    Return,    // return
    
    // 区切り文字
    LeftParen,   // (
    RightParen,  // )
    LeftBrace,   // {
    RightBrace,  // }
    Semicolon,   // ;
    Comma,       // ,
    
    // リテラル
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
}

// 文法規則の定義
#[derive(Debug)]
pub enum Statement {
    // 変数宣言
    VariableDecl {
        type_name: String,
        name: String,
        value: Expression,
    },
    
    // 関数定義
    FunctionDecl {
        is_global: bool,
        return_type: Option<String>,
        name: String,
        params: Vec<(String, String)>, // (型, 名前)
        body: Vec<Statement>,
    },
    
    // 制御文
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    BinaryOp {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
