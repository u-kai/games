use osero::game::OseroCLI;

mod masu;
mod osero;
fn main() {
    let mut osero = OseroCLI::new();
    osero.start()
}
