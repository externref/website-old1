use rand;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;

pub fn highlight_code(code: &str, lang: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts =  ThemeSet::load_defaults();;

    let syntax = ss
        .find_syntax_by_name(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-mocha.dark"]);

    let mut highlighted = String::new();
    for line in code.lines() {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss).expect("Unable to highlight.");
        let html: Vec<String> = ranges
            .iter()
            .map(|(style, text)| {
                format!(r#"<span style="{}">{}</span><br>"#, style_to_css(style), text)
            })
            .collect();
        highlighted.push_str(&format!("{}\n", html.join("")));
    }
println!("this works");
    highlighted
}

fn style_to_css(style: &Style) -> String {
    format!(
        "color:#{:02x}{:02x}{:02x};",
        style.foreground.r, style.foreground.g, style.foreground.b
    )
}

pub fn create_random_string() -> String {
    let s: String = rand::Rng::sample_iter(rand::thread_rng(), &rand::distributions::Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    return s.to_lowercase();
}
