// Structs - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name).to_string()
    }

    // Set last name
    fn set_last_name(&mut self, last_name: &str) -> &str {
        self.last_name = last_name.to_string();
        &self.last_name
    }

    // Name to tuple
    fn to_tuple(&self) -> (String, String) {
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

pub fn _run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;
    println!("Color: {} {} {} \n", c.red, c.green, c.blue);

    let mut c2 = Color2(255, 255, 255);
    c2.1 = 192;

    println!("Color: {} {} {} \n", c2.0, c2.1, c2.2);

    let mut person = Person::new("António", "Gabriel");
    person.last_name += " (AG)";
    println!("Person: {} {} \n", person.first_name, person.last_name);

    // Get full name
    println!("{}", person.full_name());

    // Set Last Name
    let last_name_added = person.set_last_name("Campos Gabriel (ACG)");
    println!("Last name added: {}", last_name_added);
    println!("{} \n", person.full_name());

    // Convert to tuple
    println!("Person Tuple: {:?}", person.to_tuple());
}
