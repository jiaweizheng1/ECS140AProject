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
    integertype = "IntergerType := [unsigned] ( char | short | int | long )",
    parameter = "Parameter := DataType Identifier",
    statement = "Statement := Assignment | Whileloop | IfStatement | ReturnStatement | (Expression ;)",
    constant = "Constant := IntConstant | FloatConstant",
    datatype = "DataType := IntergerType | FloatType",
    parameterblock = "ParameterBlock := ( [Parameter {{, Parameter}}] )",
    block = "Block := {{ {{Declaration}} {{Statement}} {{FunctionDefinition}} }}",
    functiondeclaration = "FunctionDeclaration := ParameterBlock ;",
    variabledeclaration = "VariableDDeclaration := [= Constant] ;",
    declarationtype = "DeclarationType := DataType Identifier",
    functiondefinition = "FunctionDefinition := DeclarationType ParameterBlock Block",
    maindeclaration = "MainDeclaration := void main ( ) Block",
    declaration = "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)",
    program = "Program := {{Declaration}} MainDeclaration {{FunctionDefinition}}"
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
    fn MultOperator(&mut self) -> bool
    {
        if self.t.len() > self.index && (self.t[self.index].text == "*" || self.t[self.index].text == "/")
        {
            self.index += 1;
            return true;
        }
        if self.t.len() == self.index
        {
            self.index -= 1;
        }
        println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::multoperator);
        exit(1);
    }
    fn AddOperator(&mut self) -> bool
    {
        if self.t.len() > self.index && (self.t[self.index].text  == "+" || self.t[self.index].text == "-")
        {
            self.index += 1;
            return true;
        }
        if self.t.len() == self.index
        {
            self.index -= 1;
        }
        println!("Error at Line {} Character {}. The syntax should be: {}.", self.t[self.index].line_num, self.t[self.index].char_pos, MyError::addoperator);
        exit(1);
    }

    fn solve(&mut self)
    {
        Parser::MultOperator(self);
        Parser::MultOperator(self);
        Parser::MultOperator(self);
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