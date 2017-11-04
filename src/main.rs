extern crate winput;

fn main() {
    winput::unix::InputContext::new().unwrap();
}
