use crate::FileInfo;
use log::error;

pub fn internal_tree_to_rhai(tree: &InternalTree, file_info: &FileInfo) -> String {
    match tree {
        InternalTree::Label { attrs } => {
            // handle Label
            format!("Label with attrs: {:?}", attrs)
        }
        InternalTree::Box { attrs, children } => {
            // recursively handle children
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("Box with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::CenterBox { attrs, children } => {
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("CenterBox with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Button { attrs } => format!("Button with attrs: {:?}", attrs),
        InternalTree::Image { attrs } => format!("Image with attrs: {:?}", attrs),
        InternalTree::Input { attrs } => format!("Input with attrs: {:?}", attrs),
        InternalTree::Progress { attrs } => format!("Progress with attrs: {:?}", attrs),
        InternalTree::ComboBoxText { attrs } => format!("ComboBoxText with attrs: {:?}", attrs),
        InternalTree::Slider { attrs } => format!("Slider with attrs: {:?}", attrs),
        InternalTree::Checkbox { attrs } => format!("Checkbox with attrs: {:?}", attrs),
        InternalTree::Expander { attrs, children } => {
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("Expander with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Revealer { attrs, children } => {
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("Revealer with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Scroll { attrs, children } => {
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("Scroll with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::OverLay { attrs, children } => {
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("OverLay with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Stack { attrs, children } => {
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("Stack with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::Calendar { attrs } => format!("Calendar with attrs: {:?}", attrs),
        InternalTree::ColorButton { attrs } => format!("ColorButton with attrs: {:?}", attrs),
        InternalTree::ColorChooser { attrs } => format!("ColorChooser with attrs: {:?}", attrs),
        InternalTree::CircularProgress { attrs } => format!("CircularProgress with attrs: {:?}", attrs),
        InternalTree::Graph { attrs } => format!("Graph with attrs: {:?}", attrs),
        InternalTree::Transform { attrs } => format!("Transform with attrs: {:?}", attrs),
        InternalTree::EventBox { attrs, children } => {
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("EventBox with attrs: {:?}, children: {:?}", attrs, children_str)
        }
        InternalTree::ToolTip { attrs, children } => {
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("ToolTip with attrs: {:?}, children: {:?}", attrs, children_str)
        }

        InternalTree::DefWindow { name, attrs, node } => {
            let node_str = internal_tree_to_rhai(node, file_info);
            format!("DefWindow '{}' with attrs: {:?}, node: {}", name, attrs, node_str)
        }
        InternalTree::Poll { var, attrs } => format!("Poll '{}' with attrs: {:?}", var, attrs),
        InternalTree::Listen { var, attrs } => format!("Listen '{}' with attrs: {:?}", var, attrs),
        InternalTree::Enter(children) => {
            let children_str: Vec<String> = children
                .iter()
                .map(|c| internal_tree_to_rhai(c, file_info))
                .collect();
            format!("Enter with children: {:?}", children_str)
        }
        InternalTree::Unknown => {
            error!("Unknown node in file {:?}", file_info);
            "Unknown".to_string()
        }
    }
}
