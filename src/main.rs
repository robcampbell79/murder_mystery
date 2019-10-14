use murder_mystery::create_suspects;
use murder_mystery::show_suspects;
use murder_mystery::create_mansion;
use murder_mystery::show_mansion;

fn main() {
    let s = create_suspects();

    show_suspects(s);

    let m = create_mansion();

    show_mansion(m);
}
