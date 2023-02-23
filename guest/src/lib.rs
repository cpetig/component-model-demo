use pulldown_cmark::{html, Parser};

wit_bindgen::generate!({
    path: "../wit/markdown.wit",
    world: "markdown"
});

struct Markdown;

impl markdown::Markdown for Markdown {
    fn render(input: String) -> String {
        let parser = Parser::new(&input);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        return html_output;
    }
}

export_renderer!(Markdown);
