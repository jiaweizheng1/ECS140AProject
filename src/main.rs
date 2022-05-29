use std::env;
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use custom_error::custom_error;
use std::process::exit; 

#[derive(Clone)]
enum TokenType
{
    IntConstant, 
    FloatConstant, 
    Keyword, 
    Operator, 
    Identifier, 
    Invalid
}

impl TokenType
{
    fn as_str(&self) -> &str
    {
        match &self
        {
            TokenType::IntConstant => "IntConstant",
            TokenType::FloatConstant => "FloatConstant",
            TokenType::Keyword => "Keyword",
            TokenType::Operator => "Operator",
            TokenType::Identifier => "Identifier",
            TokenType::Invalid => "Invalid"
        }
    }
}

#[derive(Clone)]
struct Token
{
    text: String,
    token_type: TokenType,
    line_num: i32,
    char_pos: i32
}

impl Token
{
    fn init(t: String, tok_type: TokenType, l: i32, c: i32) -> Token
    {
        return Token
        {
            text: t,
            token_type: tok_type,
            line_num: l,
            char_pos: c
        }
    }
}

struct CStream
{
    contents: String,
}

impl CStream
{
    fn init(in_file_name: &str) -> CStream
    {
        let mut file = File::open(in_file_name.to_string()).expect("Error Opening File");
        let mut temp_contents = String::new();
        file.read_to_string(&mut temp_contents).expect("Error Reading File");

        CStream
        {
            contents: temp_contents,
        }
    }
}

fn Scanner(input: String) -> Vec<Token>
{
    let keywords = vec!["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"].iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let operators = vec!["(", ",", ")", "{", "}", "=", "==", "<", ">", "<=", ">=", "!=", "+", "-", "*", "/", ";"].iter().map(|x| x.to_string()).collect::<Vec<String>>();

    let mut text: String = "".to_string();

    let mut token_list: Vec<Token> = Vec::new();

    let mut cur_line_num: i32 = 0;

    let mut cur_char_pos: i32 = 0;

    let mut token_char_pos: i32 = -1;

    let mut expect_new_token: bool = true;

    let mut expect_first_token_separator: bool = false;

    for i in 0..input.len()
    {

        if input.as_bytes()[i] as char == '(' || input.as_bytes()[i] as char == ')' || input.as_bytes()[i] as char == '{' || input.as_bytes()[i] as char == '}' || input.as_bytes()[i] as char == ',' || input.as_bytes()[i] as char == ';'
        {
            if text.len() > 0
            {
                for x in 0..keywords.len()
                {
                    if text == keywords[x]
                    {
                        token_list.push(Token::init(text, TokenType::Keyword, cur_line_num, token_char_pos));
                        text = "".to_string();
                    }
                }
                for x in 0..operators.len()
                {
                    if text == operators[x]
                    {
                        token_list.push(Token::init(text, TokenType::Operator, cur_line_num, token_char_pos));
                        text = "".to_string();
                    }
                }
                if text != "".to_string()
                {
                    let mut t: TokenType = TokenType::Invalid;
                    if Identifier(text.to_string())
                    {
                        t = TokenType::Identifier;
                    }
                    else if IntConstant(text.to_string())
                    {
                        t = TokenType::IntConstant;
                    }
                    else if FloatConstant(text.to_string())
                    {
                        t = TokenType::FloatConstant;
                    }
                    token_list.push(Token::init(text, t, cur_line_num, token_char_pos));
                    text = "".to_string();
                }
                token_list.push(Token::init((input.as_bytes()[i] as char).to_string(), TokenType::Operator, cur_line_num, cur_char_pos));
            }
            else
            {
                token_list.push(Token::init((input.as_bytes()[i] as char).to_string(), TokenType::Operator, cur_line_num, cur_char_pos));
            }
            expect_new_token = true;
            expect_first_token_separator = false;
        }
        else if input.as_bytes()[i] as char == '\n'
        {
            if text.len() > 0
            {
                for x in 0..keywords.len()
                {
                    if text == keywords[x]
                    {
                        token_list.push(Token::init(text, TokenType::Keyword, cur_line_num, token_char_pos));
                        text = "".to_string();
                    }
                }
                for x in 0..operators.len()
                {
                    if text == operators[x]
                    {
                        token_list.push(Token::init(text, TokenType::Operator, cur_line_num, token_char_pos));
                        text = "".to_string();
                    }
                }
                if text != "".to_string()
                {
                    let mut t: TokenType = TokenType::Invalid;
                    if Identifier(text.to_string())
                    {
                        t = TokenType::Identifier;
                    }
                    else if IntConstant(text.to_string())
                    {
                        t = TokenType::IntConstant;
                    }
                    else if FloatConstant(text.to_string())
                    {
                        t = TokenType::FloatConstant;
                    }
                    token_list.push(Token::init(text, t, cur_line_num, token_char_pos));
                    text = "".to_string();
                }
            }
            cur_line_num += 1;
            cur_char_pos = 0;
            text = "".to_string();
            expect_new_token = true;
            expect_first_token_separator = false;
            continue;
        }
        else if input.as_bytes()[i] as char == ' '
        {
            if expect_first_token_separator 
            {
                for x in 0..keywords.len()
                {
                    if text == keywords[x]
                    {
                        token_list.push(Token::init(text, TokenType::Keyword, cur_line_num, token_char_pos));
                        text = "".to_string();
                    }
                }
                for x in 0..operators.len()
                {
                    if text == operators[x]
                    {
                        token_list.push(Token::init(text, TokenType::Operator, cur_line_num, token_char_pos));
                        text = "".to_string();
                    }
                }
                if text != "".to_string()
                {
                    let mut t: TokenType = TokenType::Invalid;
                    if Identifier(text.to_string())
                    {
                        t = TokenType::Identifier;
                    }
                    else if IntConstant(text.to_string())
                    {
                        t = TokenType::IntConstant;
                    }
                    else if FloatConstant(text.to_string())
                    {
                        t = TokenType::FloatConstant;
                    }
                    token_list.push(Token::init(text, t, cur_line_num, token_char_pos));
                    text = "".to_string();
                }

                expect_first_token_separator = false;
            }

            expect_new_token = true;
        }
        else
        {
            if expect_new_token 
            {
                token_char_pos = cur_char_pos;
                expect_new_token = false;
            }

            text.push(input.as_bytes()[i] as char);

            expect_first_token_separator = true;
        }

        cur_char_pos += 1;
    }

    return token_list;
}

fn Digit(c: char) -> bool
{
    if c >= '0' && c <= '9'
    {
        return true;
    }
    return false;
}

fn Alpha(c: char) -> bool
{
    if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
    {
        return true;
    }
    return false;
}

fn IntConstant(input: String) -> bool
{
    let mut i: usize = 1;
    if input.as_bytes()[0] as char == '-' || Digit(input.as_bytes()[0] as char)
    {
        while i < input.len()
        {
            if !Digit(input.as_bytes()[i] as char)
            {
                return false;
            }
            i += 1;
        }
        return true;
    }
    return false;
}

fn FloatConstant(input: String) -> bool
{
    let mut i: usize = 1;
    if IntConstant(input.to_string())
    {
        return true;
    }
    if input.as_bytes()[0] as char == '-' || Digit(input.as_bytes()[0] as char)
    {
        while i < input.len() 
        {
            if input.as_bytes()[i] as char == '.'
            {
                i += 1;
                break;
            }
            else if !Digit(input.as_bytes()[i] as char)
            {
                return false;
            }
            i += 1;
        }
    }
    while i < input.len() 
    {
        if !Digit(input.as_bytes()[i] as char)
        {
            return false;
        }
        i += 1;
    }
    return true;
}

fn Identifier(input: String) -> bool
{
    let mut i: usize = 1;

    if input.as_bytes()[0] as char == '_' || Alpha(input.as_bytes()[0] as char)
    {
        while i < input.len()
        {
            if !(input.as_bytes()[i] as char == '_' || Alpha(input.as_bytes()[i] as char) || Digit(input.as_bytes()[i] as char))
            {
                return false;
            }
            i += 1;
        }
        return true;
    }
    return false;
}

custom_error!{pub MyError
    multoperator = "MultOperator := * | /",
    addoperator = "AddOperator := + | -",
    relationoperator = "RelationOperator := ( == ) | < | > | ( <= ) | ( >= ) | ( != )",
    factor = "Factor := ( ( Expression ) ) | Constant | (Identifier [ ( [ Expression {{, Expression}} ] ) ] )",
    term = "Term := Factor {{ MultOperator Factor }}",
    simpleexpression = "SimpleExpression := Term {{ AddOperator Term }}",
    expression = "Expression := SimpleExpression [ RelationOperator SimpleExpression ]",
    returnstatement = "ReturnStatement := return Expression ;",
    ifstatement = "IfStatement := if ( Expression ) Block",
    whileloop = "WhileLoop := while ( Expression ) Block",
    assignment = "Assignment := Identifier = {{ Identifier =}} Expression ;",
    floattype = "FloatType := float | double",
    integertype = "IntegerType := [unsigned] ( char | short | int | long )",
    parameter = "Parameter := DataType Identifier",
    statement = "Statement := Assignment | Whileloop | IfStatement | ReturnStatement | (Expression ;)",
    constant = "Constant := IntConstant | FloatConstant",
    datatype = "DataType := IntegerType | FloatType",
    parameterblock = "ParameterBlock := ( [Parameter {{, Parameter}}] )",
    block = "Block := {{ {{Declaration}} {{Statement}} {{FunctionDefinition}} }}",
    functiondeclaration = "FunctionDeclaration := ParameterBlock ;",
    variabledeclaration = "VariableDeclaration := [= Constant] ;",
    declarationtype = "DeclarationType := DataType Identifier",
    functiondefinition = "FunctionDefinition := DeclarationType ParameterBlock Block",
    maindeclaration = "MainDeclaration := void main ( ) Block",
    declaration = "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)",
    program = "Program := {{Declaration}} MainDeclaration {{FunctionDefinition}}",
    intconstant = "IntConstant := [ - ] Digit {{ Digit }}",
    floatconstant = "FloatConstant := [ - ] Digit {{ Digit }} [ . Digit {{ Digit }} ]",
    identifier = "Identifier := ( _ | Alpha ) {{ ( _ | Digit | Alpha) }}"
}

struct Parser
{
    t: Vec<Token>,
    index: usize
}

impl Parser
{
    fn init(tokens: Vec<Token>) -> Parser
    {
        return Parser
        {
            t: tokens,
            index: 0
        }
    }
    fn MultOperator(&mut self)
    {
        if self.t[self.index].text == "*" || self.t[self.index].text == "/"
        {
            self.index += 1;
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::multoperator);
            exit(1);
        }
    }
    fn AddOperator(&mut self)
    {
        if self.t[self.index].text  == "+" || self.t[self.index].text == "-"
        {
            self.index += 1;
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::addoperator);
            exit(1);
        }
    }
    fn RelationOperator(&mut self)
    {
        if self.t[self.index].text  == "==" || self.t[self.index].text == "<" || self.t[self.index].text == ">" || self.t[self.index].text == "<=" || self.t[self.index].text == ">=" || self.t[self.index].text == "!="
        {
            self.index += 1;
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::relationoperator);
            exit(1);
        }
    }
    fn FloatType(&mut self)
    {
        if self.t[self.index].text  == "float" || self.t[self.index].text == "double" 
        {
            self.index += 1;
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::floattype);
            exit(1);
        }
    }
    fn IntegerType(&mut self)
    {
        if self.t[self.index].text  == "unsigned" 
        {
            self.index += 1;

            if self.t[self.index].text  == "char" || self.t[self.index].text  == "short" || self.t[self.index].text  == "int" || self.t[self.index].text  == "long"
            {
                self.index += 1;
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::integertype);
                exit(1);
            }  
        }
        else if self.t[self.index].text  == "char" || self.t[self.index].text  == "short" || self.t[self.index].text  == "int" || self.t[self.index].text  == "long"
        {
            self.index += 1;
        } 
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::integertype);
            exit(1);
        }
    }
    fn DataType(&mut self) 
    {
        if self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd'
        {
            Parser::FloatType(self);
        }
        else if self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
        {
            Parser::IntegerType(self);
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::datatype);
            exit(1);
        }
    }
    fn IntConstant(&mut self)
    {
        let mut i: usize = 1;
        if self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char)
        {
            while i < self.t[self.index].text.len()
            {
                if !Digit(self.t[self.index].text.as_bytes()[i] as char)
                {
                    println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::intconstant);
                    exit(1);
                }
                i += 1;
            }
            self.index += 1;
        }
    }
    fn FloatConstant(&mut self)
    {
        let mut i: usize = 1;
        if self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char)
        {
            while i < self.t[self.index].text.len() 
            {
                if self.t[self.index].text.as_bytes()[i] as char == '.'
                {
                    i += 1;
                    break;
                }
                else if !Digit(self.t[self.index].text.as_bytes()[i] as char)
                {
                    println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::floatconstant);
                    exit(1);
                }
                i += 1;
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::floatconstant);
            exit(1);
        }
        while i < self.t[self.index].text.len() 
        {
            if !Digit(self.t[self.index].text.as_bytes()[i] as char)
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::floatconstant);
                exit(1);
            }
            i += 1;
        }
        self.index += 1;
    }
    fn Identifier(&mut self) 
    {
        let mut i: usize = 1;
        if self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
        {
            while i < self.t[self.index].text.len()
            {
                if !(self.t[self.index].text.as_bytes()[i] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[i] as char) || Digit(self.t[self.index].text.as_bytes()[i] as char))
                {
                    println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::identifier);
                    exit(1);
                }
                i += 1;
            }
            self.index += 1;
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::identifier);
            exit(1);
        }
    }
    fn Constant(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char)
        {
            if self.t[self.index].token_type.as_str() == "FloatConstant" || (self.t[self.index].token_type.as_str() == "Invalid" && self.t[self.index].text.contains("."))
            {
                Parser::FloatConstant(self);
            }
            else
            {
                Parser::IntConstant(self);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::constant);
            exit(1);
        }
    }
    fn Parameter(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd'
        {
            Parser::FloatType(self);
            Parser::Identifier(self);
        }
        else if self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
        {
            Parser::IntegerType(self);
            Parser::Identifier(self);
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::parameter);
            exit(1);
        }
    }
    fn DeclarationType(&mut self) 
    {
        if self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd'
        {
            Parser::FloatType(self);
            Parser::Identifier(self);
        }
        else if self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
        {
            Parser::IntegerType(self);
            Parser::Identifier(self);
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::declarationtype);
            exit(1);
        }
    }
    fn VariableDeclaration(&mut self)
    {
        if self.t[self.index].text == "="
        {
            self.index += 1;
            Parser::Constant(self);
            if self.t[self.index].text.as_bytes()[0] as char == ';'
            {
                self.index += 1;
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::variabledeclaration);
                exit(1);
            }
        }
        else if self.t[self.index].text == ";"
        {
            self.index += 1;
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::variabledeclaration);
            exit(1);
        }
    }
    fn Factor(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == '('
        {
            self.index += 1;
            Parser::Expression(self);
            if self.t[self.index].text.as_bytes()[0] as char == ')'
            {
                self.index += 1;
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::factor);
                exit(1);
            }
        }
        else if self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char)
        {
            Parser::Constant(self);
        }
        else if self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
        {
            Parser::Identifier(self);
            if self.t[self.index].text.as_bytes()[0] as char == '('
            {
                self.index += 1;
                if self.t[self.index].text.as_bytes()[0] as char == ')'
                {
                    self.index += 1;
                }
                else if self.t[self.index].text.as_bytes()[0] as char == '(' || self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char) || self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
                {
                    Parser::Expression(self);
                    if self.t[self.index].text.as_bytes()[0] as char == ')'
                    {
                        self.index += 1;
                    }
                    else if self.t[self.index].text.as_bytes()[0] as char == ','
                    {
                        while self.t[self.index].text.as_bytes()[0] as char == ','
                        {
                            self.index += 1;
                            Parser::Expression(self);
                        }
                        if self.t[self.index].text.as_bytes()[0] as char == ')'
                        {
                            self.index += 1;
                        }
                        else
                        {
                            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::factor);
                            exit(1);
                        }
                    }
                    else
                    {
                        println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::factor);
                        exit(1);
                    }
                }
                else
                {
                    println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::factor);
                    exit(1);
                }
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::factor);
            exit(1);
        }
    }
    fn Expression(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == '(' || self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char) || self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
        {
            Parser::SimpleExpression(self);
            if self.t[self.index].text.as_bytes()[0] as char  == '=' || self.t[self.index].text.as_bytes()[0] as char == '<' || self.t[self.index].text.as_bytes()[0] as char == '>' || self.t[self.index].text.as_bytes()[0] as char == '!'
            {
                Parser::RelationOperator(self);
                Parser::SimpleExpression(self);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::expression);
            exit(1);
        }
    }
    fn SimpleExpression(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == '(' || self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char) || self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
        {
            Parser::Term(self);
            if self.t[self.index].text.as_bytes()[0] as char == '+' || self.t[self.index].text.as_bytes()[0] as char == '-'
            {
                while self.t[self.index].text.as_bytes()[0] as char == '+' || self.t[self.index].text.as_bytes()[0] as char == '-'
                {
                    Parser::AddOperator(self);
                    Parser::Term(self);
                }
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::simpleexpression);
            exit(1);
        }
    }
    fn Term(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == '(' || self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char) || self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
        {
            Parser::Factor(self);
            if self.t[self.index].text.as_bytes()[0] as char == '*' || self.t[self.index].text.as_bytes()[0] as char == '/'
            {
                while self.t[self.index].text.as_bytes()[0] as char == '*' || self.t[self.index].text.as_bytes()[0] as char == '/'
                {
                    Parser::MultOperator(self);
                    Parser::Factor(self);
                }
            }

        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::term);
            exit(1);
        }
    }
    fn ReturnStatement(&mut self)
    {
        if self.t[self.index].text == "return"
        {
            self.index += 1;
            Parser::Expression(self);
            if self.t[self.index].text == ";"
            {
                self.index += 1;
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::returnstatement);
                exit(1);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::returnstatement);
            exit(1);
        }
    }
    fn IfStatement(&mut self)
    {
        if self.t[self.index].text == "if"
        {
            self.index += 1;
            if self.t[self.index].text == "("
            {
                self.index += 1;
                Parser::Expression(self);
                if self.t[self.index].text == ")"
                {
                    self.index += 1;
                    Parser::Block(self);
                }
                else
                {
                    println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::ifstatement);
                    exit(1);
                }
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::ifstatement);
                exit(1);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::ifstatement);
            exit(1);
        }
    }
    fn WhileLoop(&mut self)
    {
        if self.t[self.index].text == "while"
        {
            self.index += 1;
            if self.t[self.index].text == "("
            {
                self.index += 1;
                Parser::Expression(self);
                if self.t[self.index].text == ")"
                {
                    self.index += 1;
                    Parser::Block(self);
                }
                else
                {
                    println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::whileloop);
                    exit(1);
                }
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::whileloop);
                exit(1);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::whileloop);
            exit(1);
        }
    }
    fn Assignment(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
        {
            Parser::Identifier(self);
            if self.t[self.index].text == "="
            {
                self.index += 1;
                if self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
                {
                    while self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
                    {
                        Parser::Identifier(self);
                        if self.t[self.index].text == "="
                        {
                            self.index += 1;
                        }
                        else
                        {
                            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::assignment);
                            exit(1);
                        }
                    }
                    Parser::Expression(self);
                    if self.t[self.index].text == ";"
                    {
                        self.index += 1;
                    }
                    else
                    {
                        println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::assignment);
                        exit(1);
                    }
                }
                else if self.t[self.index].text.as_bytes()[0] as char == '(' || self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char) || self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
                {
                    Parser::Expression(self);
                    if self.t[self.index].text == ";"
                    {
                        self.index += 1;
                    }
                    else
                    {
                        println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::assignment);
                        exit(1);
                    }
                }
                else
                {
                    println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::assignment);
                    exit(1);
                }
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::assignment);
                exit(1);
            }

        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::assignment);
            exit(1);
        }
    }
    fn Statement(&mut self)
    {
        if self.t[self.index].text == "while"
        {
            Parser::WhileLoop(self);
        }
        else if self.t[self.index].text == "if"
        {
            Parser::IfStatement(self);
        }
        else if self.t[self.index].text == "return"
        {
            Parser::ReturnStatement(self);
        }
        else if self.t[self.index + 1].text == "=" && (self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char))
        {
            Parser::Assignment(self);
        }
        else if self.t[self.index].text.as_bytes()[0] as char == '(' || self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char) || self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
        {
            Parser::Expression(self);
            if self.t[self.index].text == ";"
            {
                self.index += 1;
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::statement);
                exit(1);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::statement);
            exit(1);
        }
    }
    fn ParameterBlock(&mut self)
    {
        if self.t[self.index].text == "("
        {
            self.index += 1;
            if self.t[self.index].text == ")"
            {
                self.index += 1;
            }
            else if self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd' || self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
            {
                Parser::Parameter(self);
                if self.t[self.index].text == ")"
                {
                    self.index += 1;
                }
                else if self.t[self.index].text == ","
                {   
                    while self.t[self.index].text == ","
                    {
                        self.index += 1;
                        Parser::Parameter(self);
                    }
                    if self.t[self.index].text == ")"
                    {
                        self.index += 1;
                    }
                    else
                    {
                        println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::parameterblock);
                        exit(1);
                    }
                }
                else
                {
                    println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::parameterblock);
                    exit(1);
                }
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::parameterblock);
                exit(1);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::parameterblock);
            exit(1);
        }
    }
    fn Block(&mut self)
    {
        if self.t[self.index].text == "{"
        {
            self.index += 1;
            while (self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd' || self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l') && (self.t[self.index + 2].text != "(" && self.t[self.index + 3].text != "(")
            {
                Parser::Declaration(self);
            }
            while self.t[self.index].text == "while" || self.t[self.index].text == "if" || self.t[self.index].text == "return" || self.t[self.index].text.as_bytes()[0] as char == '(' || self.t[self.index].text.as_bytes()[0] as char == '-' || Digit(self.t[self.index].text.as_bytes()[0] as char) || self.t[self.index].text.as_bytes()[0] as char == '_' || Alpha(self.t[self.index].text.as_bytes()[0] as char)
            {
                Parser::Statement(self);
            }
            while self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd' || self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l' && (self.t[self.index + 2].text == "(" || self.t[self.index + 3].text == "(")
            {
                Parser::FunctionDeclaration(self);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::block);
            exit(1);
        }
    }
    fn FunctionDeclaration(&mut self)
    {
        if self.t[self.index].text == "="
        {
            Parser::ParameterBlock(self);
            if self.t[self.index].text == ";"
            {
                self.index += 1
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::functiondeclaration);
                exit(1);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::functiondeclaration);
            exit(1);
        }
    }
    fn FunctionDefinition(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd' || self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
        {
            Parser::DeclarationType(self);
            Parser::ParameterBlock(self);
            Parser::Block(self);
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::functiondefinition);
            exit(1);
        }
    }
    fn MainDeclaration(&mut self)
    {
        if self.t[self.index].text == "void"
        {
            self.index += 1;
            if self.t[self.index].text == "main"
            {
                self.index += 1;
                if self.t[self.index].text == "("
                {
                    self.index += 1;
                    if self.t[self.index].text == ")"
                    {
                        self.index += 1;
                        Parser::Block(self);
                    }
                    else
                    {
                        println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::maindeclaration);
                        exit(1);
                    }
                }
                else
                {
                    println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::maindeclaration);
                    exit(1);
                }
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::maindeclaration);
                exit(1);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::maindeclaration);
            exit(1);
        }
    }
    fn Declaration(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd'|| self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
        {
            Parser::DeclarationType(self);
            if self.t[self.index].text == "=" || self.t[self.index].text == ";"
            {
                Parser::VariableDeclaration(self);
            } 
            else if self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd' || self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
            {
                Parser::FunctionDeclaration(self);
            }
            else
            {
                println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::declaration);
                exit(1);
            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::declaration);
            exit(1);
        }
    }
    fn Program(&mut self)
    {
        if self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd'|| self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
        {
            while self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd'|| self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
            {
                Parser::Declaration(self);
            }
            Parser::MainDeclaration(self);
            while self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd' || self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
            {
                Parser::FunctionDeclaration(self);
            }
        }
        else if self.t[self.index].text == "void"
        {
            Parser::MainDeclaration(self);
            while self.t[self.index].text.as_bytes()[0] as char == 'f' || self.t[self.index].text.as_bytes()[0] as char == 'd' || self.t[self.index].text.as_bytes()[0] as char == 'u' || self.t[self.index].text.as_bytes()[0] as char == 'c' || self.t[self.index].text.as_bytes()[0] as char == 's' || self.t[self.index].text.as_bytes()[0] as char == 'i' || self.t[self.index].text.as_bytes()[0] as char == 'l'
            {

            }
        }
        else
        {
            println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::program);
            exit(1);
        }
    }
    fn solve(&mut self)
    {
        if self.t.len() == 0 
        {
            println!("No Tokens Detected.");
            exit(1);
        }
        Parser::Program(self);
        println!("Input program is syntactacilly correct.");
    }
}

fn main() -> std::io::Result<()> {
    //collect additional arguments after "cargo run" for txt file input name
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args.len() > 2 
    {
        println!("Usage: cargo run example#.x");
        exit(1);
    }

    let f: CStream = CStream::init(&args[1]);

    //Print File Contents//
    println!("{:?}", f.contents);

    //---------IMPORTANT--------//
    //all_tokens is here!!!!!!!!//
    let all_tokens: Vec<Token> = Scanner(f.contents);

    //Print All Tokens//
    println!("\n");
    for index in 0..all_tokens.len()
    {
        println!("Token {} = {}", index, all_tokens[index].text);
        println!("Token type: {}", all_tokens[index].token_type.as_str());
        println!("Token line_num: {}", all_tokens[index].line_num);
        println!("Token char_pos: {}\n", all_tokens[index].char_pos);
    }

    let mut f: Parser = Parser::init(all_tokens.clone());
    f.solve();

    println!("Generating ours.xhtml file");
    let mut filename: String = args[1].to_string();
    filename.truncate(filename.len() - 2);
    filename.push_str("ours.xhtml");
    let mut file = File::create(filename)?;
    file.write_all(b"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n")?;
    file.write_all(b"<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">\n")?;
    file.write_all(b"<head>\n")?;
    file.write_all(b"<title>\n")?;
    file.write_all(b"X Formatted file</title>\n")?;
    file.write_all(b"</head>\n")?;
    file.write_all(b"<body bgcolor=\"navy\" text=\"orange\" link=\"orange\" vlink=\"orange\">\n")?;
    file.write_all(b"<font face=\"Courier New\">\n")?;
    file.write_all(b" ")?;

    let mut first_word: bool = true;

    for index in 0..all_tokens.len()
    {
        if index < all_tokens.len() - 1
        {
            if first_word
            {
                for i in 0..all_tokens[index].char_pos
                {
                    file.write_all(b"&nbsp;")?;
                }

                if all_tokens[index].token_type.as_str() == "Identifier"
                {
                    file.write_all(b"<font color=\"yellow\">")?;

                    file.write_all(all_tokens[index].text.as_bytes())?;

                    file.write_all(b"</font>")?;
                }
                else if all_tokens[index].token_type.as_str() == "IntConstant" || all_tokens[index].token_type.as_str() == "FloatConstant"
                {
                    file.write_all(b"<font color=\"aqua\"><b>")?;

                    file.write_all(all_tokens[index].text.as_bytes())?;

                    file.write_all(b"</b></font>")?;
                }
                else if all_tokens[index].token_type.as_str() == "Keyword" || all_tokens[index].token_type.as_str() == "Operator"
                {
                    file.write_all(b"<font color=\"white\"><b>")?;

                    if all_tokens[index].text == "<"
                    {
                        file.write_all(b"&lt;")?;
                    }
                    else if all_tokens[index].text == "<="
                    {
                        file.write_all(b"&lt=;")?;
                    }
                    else if all_tokens[index].text == ">"
                    {
                        file.write_all(b"&gt;")?;
                    }
                    else if all_tokens[index].text == ">="
                    {
                        file.write_all(b"&gt;=")?;
                    }
                    else
                    {
                        file.write_all(all_tokens[index].text.as_bytes())?;
                    }

                    file.write_all(b"</b></font>")?;
                }

                if all_tokens[index].line_num != all_tokens[index + 1].line_num
                {
                    file.write_all(b"<br />\n")?;
                    continue;
                }
                else if all_tokens[index].char_pos + all_tokens[index].text.len() as i32 != all_tokens[index + 1].char_pos
                {
                    for i in 0..(all_tokens[index + 1].char_pos - all_tokens[index].char_pos + 1)
                    {
                        file.write_all(b" ")?;
                    }
                }

                first_word = false;
            }
            else if all_tokens[index].line_num != all_tokens[index + 1].line_num
            {
                if all_tokens[index].token_type.as_str() == "Identifier"
                {
                    file.write_all(b"<font color=\"yellow\">")?;

                    file.write_all(all_tokens[index].text.as_bytes())?;

                    file.write_all(b"</font>")?;
                }
                else if all_tokens[index].token_type.as_str() == "IntConstant" || all_tokens[index].token_type.as_str() == "FloatConstant"
                {
                    file.write_all(b"<font color=\"aqua\"><b>")?;

                    file.write_all(all_tokens[index].text.as_bytes())?;

                    file.write_all(b"</b></font>")?;
                }
                else if all_tokens[index].token_type.as_str() == "Keyword" || all_tokens[index].token_type.as_str() == "Operator"
                {
                    file.write_all(b"<font color=\"white\"><b>")?;

                    if all_tokens[index].text == "<"
                    {
                        file.write_all(b"&lt;")?;
                    }
                    else if all_tokens[index].text == "<="
                    {
                        file.write_all(b"&lt=;")?;
                    }
                    else if all_tokens[index].text == ">"
                    {
                        file.write_all(b"&gt;")?;
                    }
                    else if all_tokens[index].text == ">="
                    {
                        file.write_all(b"&gt;=")?;
                    }
                    else
                    {
                        file.write_all(all_tokens[index].text.as_bytes())?;
                    }

                    file.write_all(b"</b></font>")?;
                }

                if all_tokens[index].char_pos + all_tokens[index].text.len() as i32 != all_tokens[index + 1].char_pos
                {
                    for i in 0..(all_tokens[index + 1].char_pos - all_tokens[index].char_pos + 1)
                    {
                        file.write_all(b" ")?;
                    }
                }

                file.write_all(b"<br />\n")?;

                first_word = true;
            }
            else
            {
                if all_tokens[index].token_type.as_str() == "Identifier"
                {
                    file.write_all(b"<font color=\"yellow\">")?;

                    file.write_all(all_tokens[index].text.as_bytes())?;

                    file.write_all(b"</font>")?;
                }
                else if all_tokens[index].token_type.as_str() == "IntConstant" || all_tokens[index].token_type.as_str() == "FloatConstant"
                {
                    file.write_all(b"<font color=\"aqua\"><b>")?;

                    file.write_all(all_tokens[index].text.as_bytes())?;

                    file.write_all(b"</b></font>")?;
                }
                else if all_tokens[index].token_type.as_str() == "Keyword" || all_tokens[index].token_type.as_str() == "Operator"
                {
                    file.write_all(b"<font color=\"white\"><b>")?;

                    if all_tokens[index].text == "<"
                    {
                        file.write_all(b"&lt;")?;
                    }
                    else if all_tokens[index].text == "<="
                    {
                        file.write_all(b"&lt=;")?;
                    }
                    else if all_tokens[index].text == ">"
                    {
                        file.write_all(b"&gt;")?;
                    }
                    else if all_tokens[index].text == ">="
                    {
                        file.write_all(b"&gt;=")?;
                    }
                    else
                    {
                        file.write_all(all_tokens[index].text.as_bytes())?;
                    }

                    file.write_all(b"</b></font>")?;
                }

                if all_tokens[index].char_pos + all_tokens[index].text.len() as i32 != all_tokens[index + 1].char_pos
                {
                    for i in 0..(all_tokens[index + 1].char_pos - all_tokens[index].char_pos + 1)
                    {
                        file.write_all(b" ")?;
                    }
                }
            }
        }
        else
        {
            if all_tokens[index].token_type.as_str() == "Identifier"
            {
                file.write_all(b"<font color=\"yellow\">")?;

                file.write_all(all_tokens[index].text.as_bytes())?;

                file.write_all(b"</font>")?;
            }
            else if all_tokens[index].token_type.as_str() == "IntConstant" || all_tokens[index].token_type.as_str() == "FloatConstant"
            {
                file.write_all(b"<font color=\"aqua\"><b>")?;

                file.write_all(all_tokens[index].text.as_bytes())?;

                file.write_all(b"</b></font>")?;
            }
            else if all_tokens[index].token_type.as_str() == "Keyword" || all_tokens[index].token_type.as_str() == "Operator"
            {
                file.write_all(b"<font color=\"white\"><b>")?;

                if all_tokens[index].text == "<"
                {
                    file.write_all(b"&lt;")?;
                }
                else if all_tokens[index].text == "<="
                {
                    file.write_all(b"&lt=;")?;
                }
                else if all_tokens[index].text == ">"
                {
                    file.write_all(b"&gt;")?;
                }
                else if all_tokens[index].text == ">="
                {
                    file.write_all(b"&gt;=")?;
                }
                else
                {
                    file.write_all(all_tokens[index].text.as_bytes())?;
                }

                file.write_all(b"</b></font>")?;
            }

            file.write_all(b"<br />\n")?;
        }
    }

    file.write_all(b"</font>\n")?;
    file.write_all(b"</body>\n")?;
    file.write_all(b"</html>\n")?;
    Ok(())
}