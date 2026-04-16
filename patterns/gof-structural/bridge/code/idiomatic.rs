// Bridge in idiomatic Rust — the abstraction (Message type) is
// generic over the implementation (Renderer trait). Add a new
// message type or a new renderer independently; their product space
// is handled by the generic parameter.

// ---- Implementation axis --------------------------------------------

pub trait Renderer {
    fn heading(&self, text: &str) -> String;
    fn line(&self, text: &str) -> String;
    fn urgent_prefix(&self) -> &'static str { "" }
}

pub struct PlainRenderer;
impl Renderer for PlainRenderer {
    fn heading(&self, t: &str) -> String { format!("=== {t} ===") }
    fn line(&self, t: &str)    -> String { t.to_string() }
}

pub struct HtmlRenderer;
impl Renderer for HtmlRenderer {
    fn heading(&self, t: &str) -> String { format!("<h1>{t}</h1>") }
    fn line(&self, t: &str)    -> String { format!("<p>{t}</p>") }
    fn urgent_prefix(&self)    -> &'static str { "<strong>URGENT:</strong> " }
}

pub struct MarkdownRenderer;
impl Renderer for MarkdownRenderer {
    fn heading(&self, t: &str) -> String { format!("# {t}") }
    fn line(&self, t: &str)    -> String { t.to_string() }
    fn urgent_prefix(&self)    -> &'static str { "**URGENT**: " }
}

// ---- Abstraction axis — generic over the renderer -------------------

pub struct NormalMessage<R: Renderer> {
    pub title: String,
    pub body: Vec<String>,
    pub renderer: R,
}

impl<R: Renderer> NormalMessage<R> {
    pub fn render(&self) -> String {
        let mut out = self.renderer.heading(&self.title);
        for line in &self.body {
            out.push('\n');
            out.push_str(&self.renderer.line(line));
        }
        out
    }
}

pub struct UrgentMessage<R: Renderer> {
    pub title: String,
    pub body: Vec<String>,
    pub renderer: R,
}

impl<R: Renderer> UrgentMessage<R> {
    pub fn render(&self) -> String {
        let mut out = self.renderer.heading(&self.title);
        for line in &self.body {
            out.push('\n');
            out.push_str(&self.renderer.line(&format!(
                "{}{}",
                self.renderer.urgent_prefix(),
                line
            )));
        }
        out
    }
}

fn main() {
    let body = vec!["Deploy is stuck".into(), "Rollback pending".into()];

    // Same abstraction (UrgentMessage), three renderings.
    let m_plain = UrgentMessage { title: "Alert".into(), body: body.clone(), renderer: PlainRenderer };
    let m_html  = UrgentMessage { title: "Alert".into(), body: body.clone(), renderer: HtmlRenderer  };
    let m_md    = UrgentMessage { title: "Alert".into(), body: body.clone(), renderer: MarkdownRenderer };

    println!("--- plain ---\n{}\n",    m_plain.render());
    println!("--- html ---\n{}\n",     m_html.render());
    println!("--- markdown ---\n{}\n", m_md.render());

    // Same renderer (HtmlRenderer), two abstraction types.
    let n = NormalMessage { title: "FYI".into(), body, renderer: HtmlRenderer };
    println!("--- normal html ---\n{}", n.render());
}
