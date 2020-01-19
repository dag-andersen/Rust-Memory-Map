

fn main() {
    println!("Hello, world!");
}

mod something {

    fn loadFile() -> Result<String,String> {
        panic!("Make this test fail");
    }
}