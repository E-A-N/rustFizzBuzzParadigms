enum Taits {
    long,
    tough,
    magic
}

enum Stat_Tpes {
    attack,
    defense,
    magic,
    magic_defense,
    speed,
    t
}

impl Taits {
    fn all () -> &'static [Taits] {
        return &[
            Taits::long,
            Taits::tough,
            Taits::magic,
        ]
    }

}

fn main(){
   println!("Rust says Hello to TutorialsPoint !!");
}