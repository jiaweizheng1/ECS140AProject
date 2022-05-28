/*
struct Scanner
{
    cur_line_num: i32,
    cur_char_pos: i32
}

impl Scanner
{
    fn init() -> Scanner
    {
        return Scanner
        {
            cur_line_num: 0,
            cur_char_pos: 0
        }
    }
}
*/

use std::env;
use std::io::Read;
use std::fs::File;
use std::process::exit; 

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
                    token_list.push(Token::init(text, TokenType::Identifier, cur_line_num, token_char_pos));
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
            if text.len() > 1
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
                    token_list.push(Token::init(text, TokenType::Identifier, cur_line_num, token_char_pos));
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
                    token_list.push(Token::init(text, TokenType::Identifier, cur_line_num, token_char_pos));
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

    println!("\n");
    for index in 0..token_list.len()
    {
        println!("Token {} = {}", index, token_list[index].text);
        println!("Token type: {}", token_list[index].token_type.as_str());
        println!("Token line_num: {}", token_list[index].line_num);
        println!("Token char_pos: {}\n", token_list[index].char_pos);
    }

    return token_list;
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

fn main() {
    //collect additional arguments after "cargo run" for txt file input name
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args.len() > 2 
    {
        println!("Error: Missing File Name");
        exit(1);
    }

    let mut f: CStream = CStream::init(&args[1]);
    println!("{:?}", f.contents);

    Scanner(f.contents);
}