
use std::io::{Error, ErrorKind};  

use super::{node::Node, tokenizer::{Priority, Token, Tokenizer}};

pub struct ASTBuilder<'a>{
    tokens: Tokenizer<'a>,   
    curr_token:Token 
}

impl<'a> ASTBuilder<'a> {
    pub fn new(expr:&'a str)->Result<Self,Error>{  
        let mut tokens = Tokenizer::new(expr); 
        match tokens.next(){
            Some(token)=>{
                Ok(ASTBuilder{tokens,curr_token:token}) 
            }
            None =>
                Err(Error::new(ErrorKind::Other, "error while parsing the string to tokenizer"))  
        } 
    }
    pub fn parse(&mut self)->Result<Node,Error>{
        match self.generate_ast(Priority::Default){
            Ok(ast)=>Ok(ast),
            Err(err)=>Err(err)  
        }
    }
}
impl<'a> ASTBuilder<'a> {
    fn get_next_token(&mut self)->Result<(),Error>{ 
        let next_token = match self.tokens.next(){
            Some(token)=>token,  
            None=> return Err(Error::new(ErrorKind::Other, "wanted to get the token after consuming all tokens"))
        };
        println!("\t{:?}",next_token);   
        self.curr_token = next_token;
        Ok(()) 
    }
 
    fn generate_ast(&mut self,curr_priority:Priority)->Result<Node,Error>{
        let mut left_expr = self.parse_number()?;
        while curr_priority < self.curr_token.get_priority(){ 
            if self.curr_token == Token::EOF {
                break; 
            }
            // we have to handle case when the token with end
            let right_expr = self.convert_token_to_node(left_expr)?;  
            left_expr = right_expr; 
        }
        Ok(left_expr) 
    }

    fn parse_number(&mut self)->Result<Node,Error>{
        let curr_token = self.curr_token.clone();  
        match curr_token {
            Token::Number(val)=>{
                self.get_next_token()?; 
                Ok(Node::Number(val))  
            }
            Token::Subtract =>{
                self.get_next_token()?;
                let expr = self.generate_ast(Priority::Negative)?;   
                Ok(Node::Negtive(Box::new(expr)))  
            }
            _ =>{ 
                Err(Error::new(ErrorKind::Other, "parsing a number which is not a number"))   
            }
        }
    }

    fn convert_token_to_node(&mut self,left_expr:Node)->Result<Node,Error>{ 
        let curr_token = self.curr_token.clone();
        match curr_token {
            Token::Add =>{
                self.get_next_token()?;
                let right_expr = self.generate_ast(Priority::AddSubtract)?; 
                Ok(Node::Add(Box::new(left_expr),Box::new(right_expr)))   
            }
            Token::Subtract =>{
                self.get_next_token()?;
                let right_expr = self.generate_ast(Priority::AddSubtract)?; 
                Ok(Node::Subtract(Box::new(left_expr),Box::new(right_expr)))   
            }
            Token::Multiply =>{
                self.get_next_token()?;
                let right_expr = self.generate_ast(Priority::MultiplyDivide)?; 
                 Ok(Node::Multiply(Box::new(left_expr),Box::new(right_expr)))   
            }
            Token::Divide =>{
                self.get_next_token()?; 
                let right_expr = self.generate_ast(Priority::MultiplyDivide)?;   
                Ok(Node::Divide(Box::new(left_expr),Box::new(right_expr)))    
            
            }
            _=>{
                Err(Error::new(ErrorKind::Other,"error while parsing the data convert token to node"))  
            }
        }
    }
}


