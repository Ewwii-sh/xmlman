#[derive(Debug, Clone)]
pub struct Attr {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum InternalTree {
    // === Widgets === //
    Label { attrs: Vec<Attr> },
    Box { attrs: Vec<Attr>, children: Vec<InternalTree> },
    CenterBox { attrs: Vec<Attr>, children: Vec<InternalTree> },
    Button { attrs: Vec<Attr> },
    Image { attrs: Vec<Attr> },
    Input { attrs: Vec<Attr> },
    Progress { attrs: Vec<Attr> },
    ComboBoxText { attrs: Vec<Attr> },
    Slider { attrs: Vec<Attr> },
    Checkbox { attrs: Vec<Attr> },
    Expander { attrs: Vec<Attr>, children: Vec<InternalTree> },
    Revealer { attrs: Vec<Attr>, children: Vec<InternalTree> },
    Scroll { attrs: Vec<Attr>, children: Vec<InternalTree> },
    OverLay { attrs: Vec<Attr>, children: Vec<InternalTree> },
    Stack { attrs: Vec<Attr>, children: Vec<InternalTree> },
    Calendar { attrs: Vec<Attr> },
    ColorButton { attrs: Vec<Attr> },
    ColorChooser { attrs: Vec<Attr> },
    CircularProgress { attrs: Vec<Attr> },
    Graph { attrs: Vec<Attr> },
    Transform { attrs: Vec<Attr> },
    EventBox { attrs: Vec<Attr>, children: Vec<InternalTree> },
    ToolTip { attrs: Vec<Attr>, children: Vec<InternalTree> },

    // === Top-level macros === //
    DefWindow { name: String, attrs: Vec<Attr>, node: Box<InternalTree> },
    Poll { var: String, attrs: Vec<Attr> },
    Listen { var: String, attrs: Vec<Attr> },
    Enter(Vec<InternalTree>),

    // === Special === //
    Unknown
}
