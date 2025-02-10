use std::io;

use parser::ast_builder;

mod parser;  

fn main() {
    let mut str_buf = String::new(); 
    match io::stdin().read_line(&mut str_buf)
    {
        Ok(_)=>{
            let mut ast = ast_builder::ASTBuilder::new(&str_buf).expect("error while generating the ast"); 
            println!("builded ast");
            let mut node = ast.parse().expect("error while parsing the ast");  
            println!("get node"); 
            let x = node.eval().expect("error while evaluating the node");   
            println!("{}",x); 
        }
        Err(err)=>{
            println!("error {:}",err);  
        }
    }
}   
