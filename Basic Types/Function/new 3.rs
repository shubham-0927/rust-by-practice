fn main() {
    never_return();
}

fn never_return() -> ! {
    panic!("panics")
}