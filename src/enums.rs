// Enums are types which have a few definite values

enum Movements {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movements) {
    // Perform action depending on info
    match m {
        Movements::Up => println!("Avatar moving up"),
        Movements::Down => println!("Avatar moving down"),
        Movements::Left => println!("Avatar moving left"),
        Movements::Right => println!("Avatar moving right"),
    }
}

pub fn _run() {
    let avatar1 = Movements::Up;
    let avatar2 = Movements::Down;
    let avatar3 = Movements::Left;
    let avatar4 = Movements::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
