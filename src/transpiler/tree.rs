// dead code is enabled because span is not used that much
// after converting ast to tree. 
#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Attr {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl Span {
    /// Convert Span into a Range<usize>
    /// Returns None if either start or end is None
    pub fn to_range(&self) -> Option<std::ops::Range<usize>> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => Some(start..end),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum InternalTree {
    // === Widgets === //
    Label { attrs: Vec<Attr>, span: Span },
    Box { attrs: Vec<Attr>, children: Vec<InternalTree>, span: Span },
    CenterBox { attrs: Vec<Attr>, children: Vec<InternalTree>, span: Span },
    Button { attrs: Vec<Attr>, span: Span },
    Image { attrs: Vec<Attr>, span: Span },
    Input { attrs: Vec<Attr>, span: Span },
    Progress { attrs: Vec<Attr>, span: Span },
    ComboBoxText { attrs: Vec<Attr>, span: Span },
    Slider { attrs: Vec<Attr>, span: Span },
    Checkbox { attrs: Vec<Attr>, span: Span },
    Expander { attrs: Vec<Attr>, children: Vec<InternalTree>, span: Span },
    Revealer { attrs: Vec<Attr>, children: Vec<InternalTree>, span: Span },
    Scroll { attrs: Vec<Attr>, children: Vec<InternalTree>, span: Span },
    OverLay { attrs: Vec<Attr>, children: Vec<InternalTree>, span: Span },
    Stack { attrs: Vec<Attr>, children: Vec<InternalTree>, span: Span },
    Calendar { attrs: Vec<Attr>, span: Span },
    ColorButton { attrs: Vec<Attr>, span: Span },
    ColorChooser { attrs: Vec<Attr>, span: Span },
    CircularProgress { attrs: Vec<Attr>, span: Span },
    Graph { attrs: Vec<Attr>, span: Span },
    Transform { attrs: Vec<Attr>, span: Span },
    EventBox { attrs: Vec<Attr>, children: Vec<InternalTree>, span: Span },
    ToolTip { attrs: Vec<Attr>, children: Vec<InternalTree>, span: Span },

    // === Top-level macros === //
    DefWindow { name: String, attrs: Vec<Attr>, node: Box<InternalTree>, span: Span },
    Poll { var: String, attrs: Vec<Attr>, span: Span },
    Listen { var: String, attrs: Vec<Attr>, span: Span },
    Enter { children: Vec<InternalTree>, span: Span },
}
