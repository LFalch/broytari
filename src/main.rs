use std::env::{args};

use broytari::{Phonology, file, run};

fn main() {
    let mut args = args();
    if args.len() == 1 {
        example();
    } else {
        let p = args.nth(1).unwrap();
        let stage = args.next();
        let stage = stage.as_ref().map(|s| &**s);
        let lines = file::read(p).unwrap();

        let phono = run::run(&lines, &mut [], stage);

        println!();
        phono.print();
    }
}

fn example() {
    let mut norsk = Phonology::default();
    norsk.add_category("plosive")
        .add("p")
        .add("t")
        .add("k")
        .add("b")
        .add("d")
        .add("g");
    norsk.add_category("nasal")
        .add("m")
        .add("n")
        .add("ng");
    norsk.add_category("fricative")
        .add("f")
        .add("v")
        .add("s")
        .add("sj")
        .add("kj")
        .add("h");
    norsk.add_category("approximant")
        .add("l")
        .add("j");
    norsk.add_category("rhotic")
        .add("r");
    norsk.add_category("labial")
        .add("p")
        .add("b")
        .add("m")
        .add("v");
    norsk.add_category("alveolar")
        .add("t")
        .add("d")
        .add("n")
        .add("r")
        .add("l");
    norsk.add_category("palatal")
        .add("sj")
        .add("kj")
        .add("j");
    norsk.add_category("velar")
        .add("ng")
        .add("k")
        .add("g");
    norsk.add_category("glottal")
        .add("h");
    norsk.add_feature("voiced")
        .plus("b")
        .plus("d")
        .plus("g")
        .plus("v")
        .plus("r")
        .plus("l")
        .plus("j")
        .plus("m")
        .plus("n")
        .plus("ng")
        .minus("p")
        .minus("t")
        .minus("k")
        .minus("f")
        .minus("s")
        .minus("h")
        .minus("sj")
        .minus("kj");

    norsk.print();
}
