enum SquadTraits {
    Strong,
    Tough,
    Magic
}

enum StatTypes {
    Attack,
    Defense,
    Magic,
    MagicDefense,
    Speed,
}

impl SquadTraits {
    fn all () -> &'static [SquadTraits] {
        return &[
            SquadTraits::Strong,
            SquadTraits::Tough,
            SquadTraits::Magic,
        ]
    }

}

fn main(){
    let total_traits: usize = SquadTraits::all().len();
    println!("Rust says Hello to TutorialsPoint !!");
    println!("All squad traits: {}", total_traits);
}