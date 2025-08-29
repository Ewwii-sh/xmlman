
use crate::parser::Node;
use std::cell::RefCell;
use super::tree::{Attr, InternalTree};
use crate::error::{DiagInfo, print_diag_error};
use crate::FileInfo;

pub fn node_to_internal_tree(node_ref: &RefCell<Node>, file_info: &FileInfo) -> InternalTree {
    let node = node_ref.borrow();

    // Convert attributes
    let attrs: Vec<Attr> =
        node.attributes.iter().map(|(k, v)| Attr { key: k.clone(), value: v.clone() }).collect();

    // Recursively convert children
    let children: Vec<InternalTree> =
        node.children.iter().map(|child| node_to_internal_tree(child, file_info)).collect();

    match node.name.as_str() {
        "Label" => InternalTree::Label { attrs },
        "Box" => InternalTree::Box { attrs, children },
        "CenterBox" => InternalTree::CenterBox { attrs, children },
        "Button" => InternalTree::Button { attrs },
        "Image" => InternalTree::Button { attrs },
        "Input" => InternalTree::Input { attrs },
        "Progress" => InternalTree::Progress { attrs },
        "ComboBoxText" => InternalTree::ComboBoxText { attrs },
        "Slider" => InternalTree::Slider { attrs },
        "Checkbox" => InternalTree::Checkbox { attrs },
        "Expander" => InternalTree::Expander { attrs, children },
        "Revealer" => InternalTree::Revealer { attrs, children },
        "Scroll" => InternalTree::Scroll { attrs, children },
        "OverLay" => InternalTree::OverLay { attrs, children },
        "Stack" => InternalTree::Stack { attrs, children },
        "Calendar" => InternalTree::Calendar { attrs },
        "ColorButton" => InternalTree::ColorButton { attrs },
        "ColorChooser" => InternalTree::ColorChooser { attrs },
        "CircularProgress" => InternalTree::CircularProgress { attrs },
        "Graph" => InternalTree::Graph { attrs },
        "Transform" => InternalTree::Transform { attrs },
        "EventBox" => InternalTree::EventBox { attrs, children },
        "ToolTip" => InternalTree::ToolTip { attrs, children },
        "Window" => {
            let name_attr = attrs
                .iter()
                .find(|a| a.key == "name")
                .map(|a| a.value.clone())
                .unwrap_or_else(|| "Unnamed".to_string());

            // Wrap children in a single node if needed
            let node = if children.len() == 1 {
                Box::new(children.into_iter().next().unwrap())
            } else {
                Box::new(InternalTree::Enter(children))
            };

            InternalTree::DefWindow { name: name_attr, attrs: vec![], node }
        }
        "Poll" => {
            let var_name =
                attrs.iter().find(|a| a.key == "name").map(|a| a.value.clone()).unwrap_or_default();

            InternalTree::Poll { var: var_name, attrs }
        }
        "Listen" => {
            let var_name =
                attrs.iter().find(|a| a.key == "name").map(|a| a.value.clone()).unwrap_or_default();

            InternalTree::Listen { var: var_name, attrs }
        }
        "Root" => InternalTree::Enter(children),
        unknown => {
            print_diag_error(
                Some(file_info.file_path),
                file_info.script,
                DiagInfo {
                    message: format!("Unknown XML element: '{}'", unknown),
                    label: Some("here"),
                    note: Some("This element is not valid and will be skipped."),
                    span: node.span.as_ref().map(|s| s.range()),
                },
            );

            InternalTree::Unknown
        },
    }
}
