use super::InternalTree;
use crate::FileInfo;

pub fn internal_tree_to_rhai(tree: &InternalTree, file_info: &FileInfo) -> String {
    match tree {
        InternalTree::Label { attrs, span } => {
            // handle Label
            format!("Label with attrs: {:?}", attrs)
        }
        InternalTree::Box { attrs, children, span } => {
            // recursively handle children
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("Box with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::CenterBox { attrs, children, span } => {
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("CenterBox with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Button { attrs, span } => format!("Button with attrs: {:?}", attrs),
        InternalTree::Image { attrs, span } => format!("Image with attrs: {:?}", attrs),
        InternalTree::Input { attrs, span } => format!("Input with attrs: {:?}", attrs),
        InternalTree::Progress { attrs, span } => format!("Progress with attrs: {:?}", attrs),
        InternalTree::ComboBoxText { attrs, span } => {
            format!("ComboBoxText with attrs: {:?}", attrs)
        }
        InternalTree::Slider { attrs, span } => format!("Slider with attrs: {:?}", attrs),
        InternalTree::Checkbox { attrs, span } => format!("Checkbox with attrs: {:?}", attrs),
        InternalTree::Expander { attrs, children, span } => {
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("Expander with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Revealer { attrs, children, span } => {
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("Revealer with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Scroll { attrs, children, span } => {
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("Scroll with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::OverLay { attrs, children, span } => {
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("OverLay with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Stack { attrs, children, span } => {
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("Stack with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Calendar { attrs, span } => format!("Calendar with attrs: {:?}", attrs),
        InternalTree::ColorButton { attrs, span } => format!("ColorButton with attrs: {:?}", attrs),
        InternalTree::ColorChooser { attrs, span } => {
            format!("ColorChooser with attrs: {:?}", attrs)
        }
        InternalTree::CircularProgress { attrs, span } => {
            format!("CircularProgress with attrs: {:?}", attrs)
        }
        InternalTree::Graph { attrs, span } => format!("Graph with attrs: {:?}", attrs),
        InternalTree::Transform { attrs, span } => format!("Transform with attrs: {:?}", attrs),
        InternalTree::EventBox { attrs, children, span } => {
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("EventBox with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::ToolTip { attrs, children, span } => {
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("ToolTip with attrs: {:?}, children: {:?}", attrs, children_str)
        }

        InternalTree::DefWindow { name, attrs, node, span } => {
            let node_str = internal_tree_to_rhai(node, file_info);
            format!("DefWindow '{}' with attrs: {:?}, node: {}", name, attrs, node_str)
        }
        InternalTree::Poll { var, attrs, span } => {
            format!("Poll '{}' with attrs: {:?}", var, attrs)
        }
        InternalTree::Listen { var, attrs, span } => {
            format!("Listen '{}' with attrs: {:?}", var, attrs)
        }
        InternalTree::Enter { children, span } => {
            let children_str: Vec<String> =
                children.iter().map(|c| internal_tree_to_rhai(c, file_info)).collect();
            format!("Enter with children: {:?}", children_str)
        }
    }
}
