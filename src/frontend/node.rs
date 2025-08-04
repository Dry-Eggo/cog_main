#![allow(unused)]

use crate::frontend:: {span::Spanned};

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Program (Vec<Spanned<'a, Box<Node<'a>>>>)
}
