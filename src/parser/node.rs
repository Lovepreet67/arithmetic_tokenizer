use std::error::Error;

pub enum Node{
    Number(f64),
    Multiply(Box<Node>,Box<Node>),
    Divide(Box<Node>,Box<Node>),
    Add(Box<Node>,Box<Node>),
    Subtract(Box<Node>,Box<Node>),
    Negtive(Box<Node>) 
} 

impl Node{
    pub fn eval(&mut self)->Result<f64,Box<dyn Error>>{ 
        match self {   
            Node::Number(value)=>{ 
                Ok(*value) 
            }
            Node::Add(expr1,expr2)=>{
                Ok(expr1.eval()?+expr2.eval()?)      
            }
            Node::Subtract(expr1,expr2)=>{
                Ok(expr1.eval()?-expr2.eval()?)     
            }
            Node::Multiply(expr1,expr2)=>{
                Ok(expr1.eval()?*expr2.eval()?)     
            }
            Node::Divide(expr1,expr2)=>{ 
                Ok(expr1.eval()?/expr2.eval()?)     
            }
            Node::Negtive(expr)=>{
                Ok(-expr.eval()?)  
            } 
        } 
    }
}
