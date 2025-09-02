use super::{Attr, InternalTree};
use crate::error::DiagInfo;

/// Format attributes into a Rhai-compatible map with pretty indentation
fn format_attrs(attrs: &Vec<Attr>, indent: usize) -> String {
    if attrs.is_empty() {
        return "#{}".to_string();
    }

    let indent_str = "  ".repeat(indent);
    let entries: Vec<String> = attrs
        .iter()
        .map(|a| format!("\"{}\": `{}`", a.key, a.value))
        .collect();

    if entries.len() == 1 {
        format!("#{{ {} }}", entries[0])
    } else {
        let inner = entries
            .into_iter()
            .map(|e| format!("{}  {}", indent_str, e))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("#{{\n{}\n{}}}", inner, indent_str)
    }
}

/// Special-case attribute parser for `defwindow`
fn defwidget_attrs_parser(attrs: &Vec<Attr>) -> String {
    let entries: Vec<String> = attrs
        .iter()
        .map(|a| format!("\"{}\": `{}`", a.key, a.value))
        .collect();

    format!("#{{ {} }}", entries.join(", "))
}

/// Top-level wrapper that produces a single Rhai script string
pub fn internal_tree_to_rhai(tree: &InternalTree) -> Result<String, DiagInfo<'_>> {
    let (rhai_fns, enter_call) = transpile(tree, true, 0)?;

    let mut script_parts = Vec::new();
    script_parts.extend(rhai_fns);
    script_parts.push(enter_call);

    Ok(script_parts.join("\n\n"))
}

/// Recursive transpile function with pretty-printing
fn transpile(
    tree: &InternalTree,
    is_root: bool,
    indent: usize,
) -> Result<(Vec<String>, String), DiagInfo<'_>> {
    let indent_str = "  ".repeat(indent);

    match tree {
        InternalTree::Enter { children, .. } => {
            let mut fns = Vec::new();
            let mut calls = Vec::new();

            for child in children {
                let (mut child_fns, child_call) = transpile(child, true, indent + 1)?;
                fns.append(&mut child_fns);
                calls.push(format!("{}{}", "  ".repeat(indent + 1), child_call));
            }

            let formatted = format!(
                "enter([\n{}\n{}])",
                calls.join(",\n"),
                indent_str
            );

            Ok((fns, formatted))
        }

        InternalTree::DefWindow { name, attrs, node, .. } => {
            let (mut child_fns, child_call) = transpile(node, false, indent + 1)?;
            let fn_name = format!("{}_child", name);
            let fn_def = format!(
                "fn {}() {{\n{}  {}\n{}}}",
                fn_name, indent_str, child_call, indent_str
            );
            child_fns.push(fn_def);

            let call = format!(
                "defwindow(\"{}\", {}, {}())",
                name,
                defwidget_attrs_parser(attrs),
                fn_name
            );
            Ok((child_fns, call))
        }

        InternalTree::Poll { var, attrs, span } => {
            if is_root {
                Ok((vec![], format!("poll(\"{}\", {})", var, format_attrs(attrs, indent))))
            } else {
                Err(DiagInfo {
                    message: "Orphan poll element found deep inside root".to_string(),
                    label: Some("here"),
                    note: Some("poll/listen elements should only be defined at the top of <Root>"),
                    span: span.to_range(),
                })
            }
        }

        InternalTree::Listen { var, attrs, span } => {
            if is_root {
                Ok((vec![], format!("listen(\"{}\", {})", var, format_attrs(attrs, indent))))
            } else {
                Err(DiagInfo {
                    message: "Orphan listen element found deep inside root".to_string(),
                    label: Some("here"),
                    note: Some("poll/listen elements should only be defined at the top of <Root>"),
                    span: span.to_range(),
                })
            }
        }

        // Containers with children
        InternalTree::Box { attrs, children, .. } => {
            transpile_children_container("box".to_string(), attrs, children, indent)
        }
        InternalTree::CenterBox { attrs, children, .. } => {
            transpile_children_container("centerbox".to_string(), attrs, children, indent)
        }
        InternalTree::Expander { attrs, children, .. }
        | InternalTree::Revealer { attrs, children, .. }
        | InternalTree::Scroll { attrs, children, .. }
        | InternalTree::OverLay { attrs, children, .. }
        | InternalTree::Stack { attrs, children, .. }
        | InternalTree::EventBox { attrs, children, .. }
        | InternalTree::ToolTip { attrs, children, .. } => {
            let name = format!("{:?}", tree).to_lowercase();
            transpile_children_container(name, attrs, children, indent)
        }

        // Leaf widgets
        InternalTree::Button { attrs, .. } => Ok((vec![], format!("button({})", format_attrs(attrs, indent)))),
        InternalTree::Label { attrs, .. } => Ok((vec![], format!("label({})", format_attrs(attrs, indent)))),
        InternalTree::Image { attrs, .. } => Ok((vec![], format!("image({})", format_attrs(attrs, indent)))),
        InternalTree::Input { attrs, .. } => Ok((vec![], format!("input({})", format_attrs(attrs, indent)))),
        InternalTree::Progress { attrs, .. } => Ok((vec![], format!("progress({})", format_attrs(attrs, indent)))),
        InternalTree::ComboBoxText { attrs, .. } => Ok((vec![], format!("comboboxtext({})", format_attrs(attrs, indent)))),
        InternalTree::Slider { attrs, .. } => Ok((vec![], format!("slider({})", format_attrs(attrs, indent)))),
        InternalTree::Checkbox { attrs, .. } => Ok((vec![], format!("checkbox({})", format_attrs(attrs, indent)))),
        InternalTree::Calendar { attrs, .. } => Ok((vec![], format!("calendar({})", format_attrs(attrs, indent)))),
        InternalTree::ColorButton { attrs, .. } => Ok((vec![], format!("colorbutton({})", format_attrs(attrs, indent)))),
        InternalTree::ColorChooser { attrs, .. } => Ok((vec![], format!("colorchooser({})", format_attrs(attrs, indent)))),
        InternalTree::CircularProgress { attrs, .. } => Ok((vec![], format!("circularprogress({})", format_attrs(attrs, indent)))),
        InternalTree::Graph { attrs, .. } => Ok((vec![], format!("graph({})", format_attrs(attrs, indent)))),
        InternalTree::Transform { attrs, .. } => Ok((vec![], format!("transform({})", format_attrs(attrs, indent)))),
    }
}

/// Helper for containers with children
fn transpile_children_container<'a>(
    name: String,
    attrs: &'a Vec<Attr>,
    children: &'a Vec<InternalTree>,
    indent: usize,
) -> Result<(Vec<String>, String), DiagInfo<'a>> {
    let indent_str = "  ".repeat(indent);
    let mut fns = Vec::new();
    let mut child_calls = Vec::new();

    for child in children {
        let (mut cf, cc) = transpile(child, false, indent + 1)?;
        fns.append(&mut cf);
        child_calls.push(format!("{}{}", "  ".repeat(indent + 1), cc));
    }

    let call = format!(
        "{}({}, [\n{}\n{}])",
        name,
        format_attrs(attrs, indent),
        child_calls.join(",\n"),
        indent_str
    );

    Ok((fns, call))
}
