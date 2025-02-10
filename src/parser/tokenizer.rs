use std::{iter::Peekable, str::Chars};

#[derive(PartialEq, PartialOrd)] 
pub enum Priority{
    Default,
    AddSubtract,
    MultiplyDivide,
    Negative,
    Number
}
 

#[derive(PartialEq,Clone,Debug)]    
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide, 
    Number(f64),
    EOF
}
impl Token{
    pub fn get_priority(&self)->Priority{
        match self {
            Token::Add | Token::Subtract =>{
                Priority::AddSubtract
            }
            Token::Multiply | Token::Divide =>{
                Priority::MultiplyDivide
            }
            Token::Number(_) =>{
                Priority::Number 
            } 
            _ => Priority::Default 
        } 
    }
}
pub struct Tokenizer<'a> {
    input:Peekable<Chars<'a>>  
} 

impl<'a>  Tokenizer<'a>  {
    pub fn new(input : &'a str)->Self{
          Tokenizer {input:input.chars().peekable() }    
    }  
}

impl<'a> Iterator for Tokenizer<'a>  {   
    type Item = Token;
    fn next(&mut self)->Option<Self::Item>{ 
         if let Some(curr)=self.input.next(){
            match curr {
                '+'=> {
                    Some(Token::Add)  
                }
                '-'=>{
                    Some(Token::Subtract) 
                }
                '*'=>{
                    Some(Token::Multiply)
                }
                '/'=>{
                    Some(Token::Divide)
                }
                _  =>{ 
                    if  curr.is_numeric(){
                        let mut num = curr.to_string(); 
                        while let Some(next_curr) = self.input.peek(){
                            if next_curr.is_numeric() || next_curr == &'.' {
                                num.push(*next_curr);
                               self.input.next(); 
                            }
                            else { 
                                return Some(Token::Number(num.parse().expect("error while converting this to number")));
                            }  
                        }
                         
                        Some(Token::Number(num.parse().expect("error while converting this to number")))
                    }   
                    else
                    {
                        Some(Token::EOF) 
                    } 
                }

            }
        }
        else{ 
            Some(Token::EOF)
        } 
    } 
}


