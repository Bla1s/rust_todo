use regex::Regex;
use std::fmt;
// Używamy PartialOrd oraz PartialEq aby móc porównywać kategorie
// Clone do możliwości klonowania objektów (kategorii)
// Debug do możliwości printowania kategorii
#[derive(PartialOrd, PartialEq, Clone, Debug)]
enum Category {
    Any,
    Todo,
    InProgress,
    Done,
}
// Implementacja Traitu dzięki któremu możemy printować kategorie jako string
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
// Używamy Clone do możliwości klonowania Todo
// Debug do możliwości printowania Todo
#[derive(Clone, Debug)]
struct Todo {
    name: String,
    category: Category,
}
// Wektor zawierając wszystkie obiekty typu Todo
struct TodoList {
    todos: Vec<Todo>,
}
impl TodoList {
    // Konstruktor
    fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }
    // Dodawanie Todo do wektora
    fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
    // Usuwanie Todo z wektora
    fn remove(&mut self, index: usize) {
        self.todos.remove(index);
    }
    // Wyświetlanie wszystkich Todo
    fn display_tasks(&self) {
        for (i, todo) in self.todos.iter().enumerate() {
            println!("{} - [{:?}] {}", i, todo.category, todo.name);
        }
    }
    // funkcja która służy do nadawaia priorytetów kategoriom
    fn category_priority(&self, category: &Category) -> u8 {
        match category {
            Category::Any => 0,
            Category::Todo => 1,
            Category::InProgress => 2,
            Category::Done => 3,
        }
    }
    // Wyświetlanie wszystkich Todo posortowanych po kategoriach
    fn display_tasks_by_category(&self) {
        // Tworzymy wektor krotek (indeks, todo) i sortujemy go po priorytecie kategorii
        let mut todos: Vec<(usize, &Todo)> = self.todos.iter().enumerate().collect();
        // sortujemy po priorytecie kategorii
        todos.sort_by(|a, b| {
            // porównujemy priorytety kategorii
            self.category_priority(&a.1.category)
                // cmp zwraca Ordering który jest typem enum
                .cmp(&self.category_priority(&b.1.category))
        });
        // printujemy posortowane todo
        for (i, todo) in todos {
            println!("{} - [{:?}] {}", i, todo.category, todo.name);
        }
    }
    // Funkcja która zwraca nam wszystkie Todo z pliku
    fn get_tasks_from_file(filename: &str) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(filename)?;
        let mut todos = Vec::new();
        // Regex do matchowania Todo z pliku
        let re = Regex::new(r"\[(.*?)\] (.*)").unwrap();

        for line in content.lines() {
            // przy pomocy regexa matchujemy linie z pliku
            // używamy captures w celu wyciągnięcia grup z regexa
            // matchujemy kategorię oraz nazwę
            if let Some(caps) = re.captures(line) {
                let category = match caps.get(1).map(|m| m.as_str()) {
                    Some("Any") => Category::Any,
                    Some("Todo") => Category::Todo,
                    Some("InProgress") => Category::InProgress,
                    Some("Done") => Category::Done,
                    _ => continue,
                };
                // używamy mapy żeby móc przekonwertować wartość (jeżeli jakaś jest) na String
                let name = caps
                    .get(2)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                todos.push(Todo { category, name });
            }
        }

        Ok(Self { todos })
    }
    fn update_tasks_to_file(&self, filename: &str) {
        let mut content = String::new();
        // zapisujemy wszystkie todo do stringa
        for todo in &self.todos {
            content.push_str(&format!("[{:?}] {}\n", todo.category, todo.name));
        }
        // zapisujemy stringa do pliku
        match std::fs::write(filename, content) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to write to file: {}", e),
        }
    }
}

fn main() {
    // Tworzymy nową listę todo
    let mut todo_list = TodoList::new();
    println!("Program for a simple Todo implementation");
    // pobieramy dane z pliku
    match TodoList::get_tasks_from_file("Todos.txt") {
        Ok(todos) => todo_list = todos,
        Err(e) => eprintln!("Failed to read from file: {}", e),
    }
    loop {
        println!("Choose an option:");
        println!("1. Add a todo");
        println!("2. Edit a todo");
        println!("3. Remove a todo");
        println!("4. Display all todos");
        println!("5. Display todos by category");
        println!("6. Exit");
        let mut option = String::new();
        std::io::stdin().read_line(&mut option).unwrap();
        let input = option.trim();

        match input {
            "1" => {
                println!("Enter a todo name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();
                println!("Choose the category: 1 - Any, 2 - Todo, 3 - InProgress");
                let mut category = String::new();
                std::io::stdin().read_line(&mut category).unwrap();
                let category = match category.trim() {
                    "1" => Category::Any,
                    "2" => Category::Todo,
                    "3" => Category::InProgress,
                    _ => {
                        println!("Invalid category");
                        continue;
                    }
                };
                todo_list.add(Todo { name, category });
                todo_list.update_tasks_to_file("Todos.txt");
            }
            "2" => {
                todo_list.display_tasks();
                println!("Enter the todo index to edit:");
                let mut index = String::new();
                std::io::stdin().read_line(&mut index).unwrap();
                let index = index.trim().parse::<usize>().unwrap();
                println!("What do you want to edit? 1 - Name, 2 - Category");
                let mut edit_option = String::new();
                std::io::stdin().read_line(&mut edit_option).unwrap();
                let edit_option = edit_option.trim();
                match edit_option {
                    "1" => {
                        println!("Enter a new name:");
                        let mut name = String::new();
                        std::io::stdin().read_line(&mut name).unwrap();
                        let name = name.trim().to_string();
                        todo_list.todos[index].name = name;
                    }
                    "2" => {
                        println!(
                            "Choose the category: 1 - Any, 2 - Todo, 3 - InProgress, 4 - Done"
                        );
                        let mut category = String::new();
                        std::io::stdin().read_line(&mut category).unwrap();
                        let category = match category.trim() {
                            "1" => Category::Any,
                            "2" => Category::Todo,
                            "3" => Category::InProgress,
                            "4" => Category::Done,
                            _ => {
                                println!("Invalid category");
                                continue;
                            }
                        };
                        todo_list.todos[index].category = category;
                    }
                    _ => {
                        println!("Invalid option");
                        continue;
                    }
                }
                todo_list.update_tasks_to_file("Todos.txt");
            }
            "3" => {
                todo_list.display_tasks();
                println!("Enter the todo index to remove:");
                let mut index = String::new();
                std::io::stdin().read_line(&mut index).unwrap();
                let index = index.trim().parse::<usize>().unwrap();
                todo_list.remove(index);
                todo_list.update_tasks_to_file("Todos.txt");
            }
            "4" => {
                todo_list.display_tasks();
            }
            "5" => {
                todo_list.display_tasks_by_category();
            }
            "6" => {
                println!("Exiting program!");
                break;
            }
            _ => {
                println!("Invalid option");
                continue;
            }
        }
    }
}
