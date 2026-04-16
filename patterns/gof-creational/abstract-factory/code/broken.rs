// Broken example — trying to mix products from two different kits.
// Abstract Factory's defining virtue is that this is rejected at
// compile time: `WindowsKit::Button` and `MacKit::Button` are
// distinct concrete types bound through associated types.
//
// This file is expected to FAIL to compile.

pub trait Button     { fn render(&self, label: &str) -> String; }
pub trait Checkbox   { fn render(&self, label: &str, checked: bool) -> String; }

pub trait UiKit {
    type Button: Button;
    type Checkbox: Checkbox;
    fn button(&self) -> Self::Button;
    fn checkbox(&self) -> Self::Checkbox;
}

pub struct WindowsKit;
pub struct WindowsButton;
pub struct WindowsCheckbox;
impl Button   for WindowsButton   { fn render(&self, l: &str) -> String { format!("[ {l} ]") } }
impl Checkbox for WindowsCheckbox { fn render(&self, l: &str, c: bool) -> String { format!("[{}] {l}", if c {"x"} else {" "}) } }
impl UiKit for WindowsKit {
    type Button = WindowsButton;
    type Checkbox = WindowsCheckbox;
    fn button(&self) -> WindowsButton { WindowsButton }
    fn checkbox(&self) -> WindowsCheckbox { WindowsCheckbox }
}

pub struct MacKit;
pub struct MacButton;
pub struct MacCheckbox;
impl Button   for MacButton   { fn render(&self, l: &str) -> String { format!("( {l} )") } }
impl Checkbox for MacCheckbox { fn render(&self, l: &str, c: bool) -> String { format!("( {} ) {l}", if c {"●"} else {"○"}) } }
impl UiKit for MacKit {
    type Button = MacButton;
    type Checkbox = MacCheckbox;
    fn button(&self) -> MacButton { MacButton }
    fn checkbox(&self) -> MacCheckbox { MacCheckbox }
}

// A function that accepts "the button from this kit and the
// checkbox from THIS SAME kit" naturally prevents mixing.
fn render_pair<K: UiKit>(kit: &K) -> String {
    format!("{}\n{}",
        kit.button().render("OK"),
        kit.checkbox().render("wat", true))
}

fn main() {
    // OK — both from WindowsKit.
    println!("{}", render_pair(&WindowsKit));

    // Try to pass a WindowsButton with a MacCheckbox manually —
    // there's no signature in our API that allows it because every
    // `Kit: UiKit` bound ties both products to the same Kit.
    let wb: WindowsButton = WindowsKit.button();
    let mc: MacCheckbox   = MacKit.checkbox();

    // And if you tried to smuggle them into a tuple that expects
    // one kit's pair, you'd get E0308. Uncomment to see it:
    //
    //   let _pair: (<WindowsKit as UiKit>::Button,
    //               <WindowsKit as UiKit>::Checkbox) = (wb, mc);
    //                                                   -- expected
    //                                                      WindowsCheckbox,
    //                                                      found MacCheckbox
    //                                                      (E0308)
    //
    // The error is the whole point of the pattern.
    println!("{} {}", wb.render("x"), mc.render("y", true));
}
