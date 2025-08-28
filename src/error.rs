use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use std::ops::Range;

pub struct DiagInfo<'a> {
    pub message: String,
    pub label: Option<&'a str>,
    pub note: Option<&'a str>,
    pub span: Option<Range<usize>>,
}

pub fn print_diag_error(file_path: Option<&str>, source_code: &str, info: DiagInfo) {
    let file_id = file_path.unwrap_or("<code>");
    let span = match info.span {
        Some(s) => s,
        None => 0..1,
    };

    let mut colors = ColorGenerator::new();

    let a = colors.next();
    let out = Color::Fixed(81);

    let mut report =
        Report::build(ReportKind::Error, (file_id, span.clone())).with_message(info.message);

    if let Some(label) = info.label {
        report = report.with_label(Label::new((file_id, span)).with_message(label).with_color(a))
    };

    if let Some(note) = info.note {
        report = report.with_note(note.fg(out))
    };

    // print error
    report.finish().print((file_id, Source::from(source_code))).unwrap();
}
