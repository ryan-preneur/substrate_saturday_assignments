fn main() {
    never_return();
}

fn never_return() -> ! {
    panic!("I return nothing!")
}