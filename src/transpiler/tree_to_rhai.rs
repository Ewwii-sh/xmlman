use super::{Attr, InternalTree};
use crate::error::DiagInfo;

fn format_attrs(attrs: &Vec<Attr>) -> String {
    let entries: Vec<String> =
        attrs.iter().map(|a| format!("\"{}\": `{}`", a.key, a.value)).collect();
    format!("#{{ {} }}", entries.join(", "))
}

/// Special case attribute parser for `defwidget`.
///
/// A special case parser is needed because in Rhai, we can use
/// a `Map` on certain properties like `geometry` and `reserve`
/// which xml lacks due to its flat atribute style.
fn defwidget_attrs_parser(attrs: &Vec<Attr>) -> String {
    let entries: Vec<String> =
        attrs.iter().map(|a| format!("\"{}\": `{}`", a.key, a.value)).collect();

    for entry in &entries {
        let name_attr = attrs
            .iter()
            .find(|a| a.key == "name")
            .map(|a| a.value.clone())
            .unwrap_or_else(|| "Unnamed".to_string());
    }

    format!("#{{ {} }}", entries.join(", "))
}

/// Top-level wrapper that produces a single Rhai script string
pub fn internal_tree_to_rhai(tree: &InternalTree) -> Result<String, DiagInfo<'_>> {
    let (rhai_fns, enter_call) = transpile(tree, true)?;

    let mut script_parts = Vec::new();
    script_parts.extend(rhai_fns);
    script_parts.push(enter_call);

    Ok(script_parts.join("\n\n"))
}

/// Recursive transpile function
fn transpile(tree: &InternalTree, is_root: bool) -> Result<(Vec<String>, String), DiagInfo<'_>> {
    match tree {
        InternalTree::Enter { children, .. } => {
            let mut fns = Vec::new();
            let mut calls = Vec::new();

            for child in children {
                match child {
                    InternalTree::Poll { .. }
                    | InternalTree::Listen { .. }
                    | InternalTree::DefWindow { .. } => {
                        let (mut child_fns, child_call) = transpile(child, true)?;
                        fns.append(&mut child_fns);
                        calls.push(child_call);
                    }
                    _ => {
                        // Other nodes inside defwindow children are handled in their own fn
                    }
                }
            }

            let formatted = format!("enter([\n  {}\n])", calls.join(",\n  "));
            Ok((fns, formatted))
        }

        InternalTree::DefWindow { name, attrs, node, .. } => {
            let (mut child_fns, child_call) = transpile(node, false)?;
            let fn_name = format!("{}_child", name);
            let fn_def = format!("fn {}() {{\n  {}\n}}", fn_name, child_call);
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
                Ok((vec![], format!("poll(\"{}\", {})", var, format_attrs(attrs))))
            } else {
                return Err(DiagInfo {
                    message: "Orphan poll element found deep inside root".to_string(),
                    label: Some("here"),
                    note: Some("poll/listen elements should only be defined at the top of <Root>"),
                    span: span.to_range(),
                });
            }
        }

        InternalTree::Listen { var, attrs, span } => {
            if is_root {
                Ok((vec![], format!("listen(\"{}\", {})", var, format_attrs(attrs))))
            } else {
                return Err(DiagInfo {
                    message: "Orphan listen element found deep inside root".to_string(),
                    label: Some("here"),
                    note: Some("poll/listen elements should only be defined at the top of <Root>"),
                    span: span.to_range(),
                });
            }
        }

        // Containers with children
        InternalTree::Box { attrs, children, .. } => {
            let mut fns = Vec::new();
            let mut child_calls = Vec::new();
            for child in children {
                let (mut cf, cc) = transpile(child, false)?;
                fns.append(&mut cf);
                child_calls.push(cc);
            }
            Ok((fns, format!("box({}, [{}])", format_attrs(attrs), child_calls.join(",\n  "))))
        }

        InternalTree::CenterBox { attrs, children, .. } => {
            let mut fns = Vec::new();
            let mut child_calls = Vec::new();
            for child in children {
                let (mut cf, cc) = transpile(child, false)?;
                fns.append(&mut cf);
                child_calls.push(cc);
            }
            Ok((
                fns,
                format!("centerbox({}, [{}])", format_attrs(attrs), child_calls.join(",\n  ")),
            ))
        }

        // Leaf widgets
        InternalTree::Button { attrs, .. } => {
            Ok((vec![], format!("button({})", format_attrs(attrs))))
        }
        InternalTree::Label { attrs, .. } => {
            Ok((vec![], format!("label({})", format_attrs(attrs))))
        }
        InternalTree::Image { attrs, .. } => {
            Ok((vec![], format!("image({})", format_attrs(attrs))))
        }
        InternalTree::Input { attrs, .. } => {
            Ok((vec![], format!("input({})", format_attrs(attrs))))
        }
        InternalTree::Progress { attrs, .. } => {
            Ok((vec![], format!("progress({})", format_attrs(attrs))))
        }
        InternalTree::ComboBoxText { attrs, .. } => {
            Ok((vec![], format!("comboboxtext({})", format_attrs(attrs))))
        }
        InternalTree::Slider { attrs, .. } => {
            Ok((vec![], format!("slider({})", format_attrs(attrs))))
        }
        InternalTree::Checkbox { attrs, .. } => {
            Ok((vec![], format!("checkbox({})", format_attrs(attrs))))
        }
        InternalTree::Calendar { attrs, .. } => {
            Ok((vec![], format!("calendar({})", format_attrs(attrs))))
        }
        InternalTree::ColorButton { attrs, .. } => {
            Ok((vec![], format!("colorbutton({})", format_attrs(attrs))))
        }
        InternalTree::ColorChooser { attrs, .. } => {
            Ok((vec![], format!("colorchooser({})", format_attrs(attrs))))
        }
        InternalTree::CircularProgress { attrs, .. } => {
            Ok((vec![], format!("circularprogress({})", format_attrs(attrs))))
        }
        InternalTree::Graph { attrs, .. } => {
            Ok((vec![], format!("graph({})", format_attrs(attrs))))
        }
        InternalTree::Transform { attrs, .. } => {
            Ok((vec![], format!("transform({})", format_attrs(attrs))))
        }
        InternalTree::Expander { attrs, children, .. }
        | InternalTree::Revealer { attrs, children, .. }
        | InternalTree::Scroll { attrs, children, .. }
        | InternalTree::OverLay { attrs, children, .. }
        | InternalTree::Stack { attrs, children, .. }
        | InternalTree::EventBox { attrs, children, .. }
        | InternalTree::ToolTip { attrs, children, .. } => {
            let mut fns = Vec::new();
            let mut child_calls = Vec::new();
            for child in children {
                let (mut cf, cc) = transpile(child, false)?;
                fns.append(&mut cf);
                child_calls.push(cc);
            }
            let name = format!("{:?}", tree).to_lowercase();
            Ok((fns, format!("{}({}, [{}])", name, format_attrs(attrs), child_calls.join(",\n  "))))
        }
    }
}
