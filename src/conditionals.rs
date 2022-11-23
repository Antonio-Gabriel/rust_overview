pub fn _run() {
    let age = 26;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // If/Else
    if age >= 20 && check_id || knows_person_of_age {
        println!("You can enter!");
    } else {
        println!("Sorry, but you connot enter");
    }

    // Shorthand If
    let is_of_age = if age >= 20 { true } else { false };
    println!("{:?}", is_of_age);
}
