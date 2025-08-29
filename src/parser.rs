//! The parser uses both _xmlparser_ and _xml-rs_ to parse xml.
//! Both plays a crutial role in making _xmlman_ elegant.
//! _xmlparser's_ role in parsing is validating the script,
//! finding errors, generating spans etc.
//! And _xml-rs's_ role is generating the AST (Abstract Syntax Tree).
//!
//! Combining both of them gives xmlman the best of both worlds.
//! Good errors throught spans (start..end) and fast AST parsing.

use super::FileInfo;
use crate::error::{DiagInfo, print_diag_error};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use xml::reader::{EventReader, XmlEvent};
use xmlparser::{StrSpan, Token, Tokenizer};

/// [`Node`] is a structure created by _xmlparser_.
/// It contains essential info such as the xml element
/// name, attributes, children and span.
#[derive(Debug)]
pub struct Node<'a> {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub span: Option<StrSpan<'a>>,
}

pub fn parse_xml<'a>(file_info: &FileInfo<'a>) -> Result<Rc<RefCell<Node<'a>>>, Box<dyn Error>> {
    // First pass: xmlparser to gather spans and detect errors
    let mut token_spans = Vec::new();
    let tokenizer = Tokenizer::from(file_info.script);
    for token_result in tokenizer {
        match token_result {
            Ok(token) => match token {
                Token::ElementStart { span, .. }
                | Token::ElementEnd { span, .. }
                | Token::Attribute { span, .. }
                | Token::ProcessingInstruction { span, .. } => {
                    token_spans.push(span);
                }
                _ => {}
            },
            Err(e) => {
                // since its a parse error, we dont have span
                // we need to make up a span
                let pos = e.pos();

                let mut start_byte = 0;
                let mut current_row = 1;

                for line in file_info.script.lines() {
                    if current_row == pos.row {
                        start_byte += line
                            .char_indices()
                            .nth((pos.col - 1) as usize)
                            .map(|(byte_idx, _)| byte_idx)
                            .unwrap_or(line.len());
                        break;
                    } else {
                        start_byte += line.len() + 1;
                        current_row += 1;
                    }
                }

                print_diag_error(
                    Some(file_info.file_path),
                    file_info.script,
                    DiagInfo {
                        message: e.to_string(),
                        label: Some("here"),
                        note: None,
                        span: Some(start_byte..start_byte + 1),
                    },
                );

                return Err(Box::new(e));
            }
        }
    }

    // Second pass: xml-rs to build AST
    let parser = EventReader::from_str(file_info.script);
    let mut root_node: Option<Rc<RefCell<Node<'a>>>> = None;
    let mut node_stack: Vec<Rc<RefCell<Node<'a>>>> = Vec::new();
    let mut token_iter = Tokenizer::from(file_info.script).filter_map(|r| r.ok());

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let span = loop {
                    match token_iter.next() {
                        Some(Token::ElementStart { span, .. }) => break Some(span),
                        Some(_) => continue,
                        None => break None,
                    }
                };

                let node = Rc::new(RefCell::new(Node {
                    name: name.local_name,
                    attributes: attributes
                        .into_iter()
                        .map(|attr| (attr.name.local_name, attr.value))
                        .collect(),
                    children: vec![],
                    span,
                }));

                if let Some(parent) = node_stack.last() {
                    parent.borrow_mut().children.push(node.clone());
                } else {
                    root_node = Some(node.clone());
                }

                node_stack.push(node);
            }
            Ok(XmlEvent::EndElement { .. }) => {
                node_stack.pop();
            }
            Ok(XmlEvent::Characters(_text)) => {}
            Ok(_) => {}

            Err(e) => {
                // parser-level errors
                print_diag_error(
                    Some(file_info.file_path),
                    file_info.script,
                    DiagInfo { message: e.to_string(), label: None, note: None, span: None },
                );
                return Err(Box::new(e));
            }
        }
    }

    Ok(root_node.ok_or_else(|| {
        print_diag_error(
            Some(file_info.file_path),
            file_info.script,
            DiagInfo {
                message: "XML should have a root node".to_string(),
                label: None,
                note: None,
                span: None,
            },
        );
        "XML should have a root node"
    })?)
}
