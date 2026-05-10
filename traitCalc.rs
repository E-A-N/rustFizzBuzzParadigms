enum Taits {
    long,
    tough,
    magic
}

impl Taits {
    fn all_taits () -> (Taits, 3) {
        let result : (Taits, 3) = [
            Taits::long,
            Taits::tough,
            Taits::magic,
        ]

        return result;
    }

}

fn main(){
   println!("Rust says Hello to TutorialsPoint !!");
}