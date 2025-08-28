use crate::error::{DiagInfo, print_diag_error};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use xml::reader::{EventReader, XmlEvent};
use xmlparser::{StrSpan, Token, Tokenizer};

#[derive(Debug)]
pub struct Node<'a> {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub span: Option<StrSpan<'a>>,
}

pub fn parse_xml<'a>(
    xml_content: &'a str,
    file_path: &'a str,
) -> Result<Rc<RefCell<Node<'a>>>, Box<dyn Error>> {
    // First pass: xmlparser to gather spans and detect errors
    let mut token_spans = Vec::new();
    let tokenizer = Tokenizer::from(xml_content);
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

                for line in xml_content.lines() {
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
                    Some(file_path),
                    xml_content,
                    DiagInfo {
                        message: e.to_string(),
                        label: Some("here"),
                        note: None,
                        span: Some(start_byte..start_byte + 1),
                    },
                )
            }
        }
    }

    // Second pass: xml-rs to build AST
    let parser = EventReader::from_str(xml_content);
    let mut root_node: Option<Rc<RefCell<Node<'a>>>> = None;
    let mut node_stack: Vec<Rc<RefCell<Node<'a>>>> = Vec::new();
    let mut token_iter = Tokenizer::from(xml_content).filter_map(|r| r.ok());

    for e in parser {
        match e? {
            XmlEvent::StartElement { name, attributes, .. } => {
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
            XmlEvent::EndElement { .. } => {
                node_stack.pop();
            }
            XmlEvent::Characters(_text) => {}
            _ => {}
        }
    }

    Ok(root_node.expect("XML should have a root node"))
}
