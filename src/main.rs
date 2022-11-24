use std::ops::IndexMut;

mod arrays;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod matchs;
mod pointers_ref;
mod print;
mod strings;
mod structs;
mod traits;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    let mut persons: Vec<&str> = vec![];
    persons.push("António");
    persons.push("Kiala");

    println!("Index: {}", persons.index_mut(0));

    println!("Index: {:?}", persons.get_mut(1));

    println!("Persons: {:?}", persons);

    for (index, person) in persons.iter_mut().enumerate() {
        println!("Id: {}, Name: {}", index, *person);
    }

    let mut filter_by_name = persons.iter_mut().filter(|x| **x == "Kiala");

    println!("Name: {:?}", filter_by_name.next());

    println!("Hello, world!");
    print::_run();
    vars::_run();
    types::_run();
    strings::_run();
    tuples::_run();
    arrays::_run();
    vectors::_run();
    conditionals::_run();
    loops::_run();
    functions::_run();
    pointers_ref::_run();
    structs::_run();
    enums::_run();
    matchs::_run();
    traits::_run();
}
