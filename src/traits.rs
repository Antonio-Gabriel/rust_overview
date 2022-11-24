struct Person {
    id: usize,
    name: String,
    email: String,
}

struct RepositoryInMemory {
    persons: Vec<Person>,
}

trait GenericRepositoryInterface {
    fn create(&mut self, name: &str, email: &str);
    fn get(&self) -> &Vec<Person>;
}

impl RepositoryInMemory {
    fn new(repository: Vec<Person>) -> Self {
        RepositoryInMemory {
            persons: repository,
        }
    }

    fn save(&mut self, person: Person) {
        self.persons.push(person);
    }

    fn get(&self) -> &Vec<Person> {
        &self.persons
    }
}

impl GenericRepositoryInterface for RepositoryInMemory {
    fn create(&mut self, name: &str, email: &str) {
        let new_id = self.get().len() + 1;

        self.save(Person {
            id: new_id,
            name: String::from(name),
            email: String::from(email),
        });
    }

    fn get(&self) -> &Vec<Person> {
        &self.get()
    }
}

pub fn _run() {
    let persons_db: Vec<Person> = Vec::new();

    let mut memory_repository = RepositoryInMemory::new(persons_db);
    memory_repository.create("António Gabriel", "antoniogabriel@gmail.com");
    memory_repository.create("António Gabriel", "antoniogabriel@gmail.com");

    for person in memory_repository.get() {
        println!(
            "Id: {0}, Name: {1}, Email: {2}",
            person.id, person.name, person.email
        );
    }    
}
