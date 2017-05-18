fn main() {
    println!("{}", check());
}

fn check() -> bool {
    std::mem::size_of::<usize>() == 8
}

#[test]
fn ok() {
    assert!(check());
}
