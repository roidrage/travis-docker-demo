fn main() {
    println!("{}", check());
}

#[cfg(target_arch = "x86_64")]
fn check() -> bool {
    std::mem::size_of::<usize>() == 8
}

#[cfg(not(target_arch = "x86_64"))]
fn check() -> bool {
    std::mem::size_of::<usize>() == 4
}

#[test]
fn ok() {
    assert!(check());
}
