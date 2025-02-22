
use crate::scanner::Token;

#[derive(Debug)]

pub enum TreeNode {
    Number(f64),
    Identifier(String),
    BinaryOp(Box<TreeNode>,Token,Box<TreeNode>)
}