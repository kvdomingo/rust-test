fn funky(arr: &[u32]) -> u32 {
    (*arr).len() as u32
}

fn main() {
    let x = &[2, 4, 8, 16, 32];
    let f = funky(x);
    println!("{f}");
}
