use super::tree::{Attr, InternalTree, Span};
use crate::FileInfo;
use crate::error::DiagInfo;
use crate::parser::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub fn node_to_internal_tree<'a>(
    node_ref: Rc<RefCell<Node<'a>>>,
    file_info: &'a FileInfo,
) -> Result<InternalTree, DiagInfo<'a>> {
    let node = node_ref.borrow();

    // Convert attributes
    let mut attrs: Vec<Attr> =
        node.attributes.iter().map(|(k, v)| Attr { key: k.clone(), value: v.clone() }).collect();

    // Recursively convert children
    let children: Vec<InternalTree> = node
        .children
        .iter()
        .cloned()
        .map(|child| node_to_internal_tree(child, file_info))
        .collect::<Result<_, _>>()?;

    // Convert spans
    let span = Span {
        start: node.span.as_ref().map(|s| s.start()),
        end: node.span.as_ref().map(|s| s.end()),
    };

    match node.name.as_str() {
        "Label" => Ok(InternalTree::Label { attrs, span }),
        "Box" => Ok(InternalTree::Box { attrs, children, span }),
        "CenterBox" => Ok(InternalTree::CenterBox { attrs, children, span }),
        "Button" => Ok(InternalTree::Button { attrs, span }),
        "Image" => Ok(InternalTree::Image { attrs, span }),
        "Input" => Ok(InternalTree::Input { attrs, span }),
        "Progress" => Ok(InternalTree::Progress { attrs, span }),
        "ComboBoxText" => Ok(InternalTree::ComboBoxText { attrs, span }),
        "Slider" => Ok(InternalTree::Slider { attrs, span }),
        "Checkbox" => Ok(InternalTree::Checkbox { attrs, span }),
        "Expander" => Ok(InternalTree::Expander { attrs, children, span }),
        "Revealer" => Ok(InternalTree::Revealer { attrs, children, span }),
        "Scroll" => Ok(InternalTree::Scroll { attrs, children, span }),
        "OverLay" => Ok(InternalTree::OverLay { attrs, children, span }),
        "Stack" => Ok(InternalTree::Stack { attrs, children, span }),
        "Calendar" => Ok(InternalTree::Calendar { attrs, span }),
        "ColorButton" => Ok(InternalTree::ColorButton { attrs, span }),
        "ColorChooser" => Ok(InternalTree::ColorChooser { attrs, span }),
        "CircularProgress" => Ok(InternalTree::CircularProgress { attrs, span }),
        "Graph" => Ok(InternalTree::Graph { attrs, span }),
        "Transform" => Ok(InternalTree::Transform { attrs, span }),
        "EventBox" => Ok(InternalTree::EventBox { attrs, children, span }),
        "ToolTip" => Ok(InternalTree::ToolTip { attrs, children, span }),
        "Window" => {
            let name_attr = match attrs.iter().find(|a| a.key == "name").map(|a| a.value.clone()) {
                Some(a) => a,
                None => {
                    return Err(DiagInfo {
                        message: format!("A window without a name was found!"),
                        label: Some("Add a name attribute in this element."),
                        note: None,
                        span: node.span.as_ref().map(|s| s.range()),
                    });
                }
            };

            attrs.retain(|a| a.key != "name");

            let node = if children.len() == 1 {
                Box::new(children.into_iter().next().unwrap())
            } else {
                return Err(DiagInfo {
                    message: format!("Found a window with multiple children."),
                    label: Some("here"),
                    note: Some("A window much contain exactly 1 child."),
                    span: node.span.as_ref().map(|s| s.range()),
                });
            };

            Ok(InternalTree::DefWindow { name: name_attr, attrs, node, span })
        }
        "Poll" => {
            let var_name =
                attrs.iter().find(|a| a.key == "name").map(|a| a.value.clone()).unwrap_or_default();

            Ok(InternalTree::Poll { var: var_name, attrs, span })
        }
        "Listen" => {
            let var_name =
                attrs.iter().find(|a| a.key == "name").map(|a| a.value.clone()).unwrap_or_default();

            Ok(InternalTree::Listen { var: var_name, attrs, span })
        }
        "Root" => Ok(InternalTree::Enter { children, span }),
        unknown => {
            return Err(DiagInfo {
                message: format!("Unknown XML element: '{}'", unknown),
                label: Some("here"),
                note: None,
                span: node.span.as_ref().map(|s| s.range()),
            });
        }
    }
}
