// Abstract Factory in idiomatic Rust — a trait with multiple
// associated types, each bound to its own product trait. One
// parameter `Kit: UiKit` gives you the whole family and the
// compiler locks you to consistent products.
//
// Sealing the trait (see rust-idiomatic/sealed-trait) is usually the
// right move too: otherwise downstream can ship rogue Kits.

// ---- Product traits --------------------------------------------------

pub trait Button {
    fn render(&self, label: &str) -> String;
}
pub trait Checkbox {
    fn render(&self, label: &str, checked: bool) -> String;
}

// ---- The abstract factory -------------------------------------------

pub trait UiKit {
    type Button: Button;
    type Checkbox: Checkbox;

    fn button(&self) -> Self::Button;
    fn checkbox(&self) -> Self::Checkbox;
}

// ---- Windows family --------------------------------------------------

pub struct WindowsKit;
pub struct WindowsButton;
pub struct WindowsCheckbox;

impl Button for WindowsButton {
    fn render(&self, label: &str) -> String {
        format!("[ {label} ]")                            // chonky Windows button
    }
}
impl Checkbox for WindowsCheckbox {
    fn render(&self, label: &str, checked: bool) -> String {
        format!("[{}] {label}", if checked { "x" } else { " " })
    }
}
impl UiKit for WindowsKit {
    type Button = WindowsButton;
    type Checkbox = WindowsCheckbox;
    fn button(&self) -> WindowsButton { WindowsButton }
    fn checkbox(&self) -> WindowsCheckbox { WindowsCheckbox }
}

// ---- Mac family ------------------------------------------------------

pub struct MacKit;
pub struct MacButton;
pub struct MacCheckbox;

impl Button for MacButton {
    fn render(&self, label: &str) -> String {
        format!("( {label} )")                            // rounded Mac button
    }
}
impl Checkbox for MacCheckbox {
    fn render(&self, label: &str, checked: bool) -> String {
        format!("( {} ) {label}", if checked { "●" } else { "○" })
    }
}
impl UiKit for MacKit {
    type Button = MacButton;
    type Checkbox = MacCheckbox;
    fn button(&self) -> MacButton { MacButton }
    fn checkbox(&self) -> MacCheckbox { MacCheckbox }
}

// ---- Generic client code — one Kit parameter picks the whole family -

pub fn render_form<K: UiKit>(kit: &K) -> String {
    let b = kit.button();
    let c = kit.checkbox();
    let mut out = String::new();
    out.push_str(&b.render("Submit"));
    out.push('\n');
    out.push_str(&c.render("Remember me", true));
    out.push('\n');
    out.push_str(&c.render("Subscribe to newsletter", false));
    out
}

fn main() {
    println!("--- Windows ---\n{}", render_form(&WindowsKit));
    println!("\n--- Mac ---\n{}", render_form(&MacKit));
}
