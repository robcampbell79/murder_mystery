use murder_mystery::Suspect;
use murder_mystery::create_suspects;
use murder_mystery::show_suspects;
use murder_mystery::Mansion;
use murder_mystery::create_mansion;
use murder_mystery::show_mansion;
use murder_mystery::MurderWeapon;
use murder_mystery::weapons_list;
use murder_mystery::Guilty;
use murder_mystery::show_guilty;
use murder_mystery::roll_dice;

fn main() {

    let s = create_suspects();

    show_suspects(&s);

    let m = create_mansion();

    show_mansion(&m);

    let w = weapons_list();

    let guilty = Guilty::create_guilty(s, m.rooms, w.weapons);

    show_guilty(&guilty);

    roll_dice();
}
