use std::io;

fn main() {
    use std::collections::HashMap;

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    let mut choice = String::new();

    while choice.trim().ne("q") {
        choice.clear();

        println!("What do you want to do?");
        println!("(a)dd employee to department");
        println!("(l)ist for a department");
        println!("(s)how all people by department");
        println!("(q)uit");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read your choice");

        match choice.trim() {
            "a" => {
                let mut department = String::new();

                println!("\nWhich department?");

                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read your department");

                let department = department.trim().to_lowercase();
                let department_entry = departments.entry(department).or_insert_with(Vec::new);

                let mut employee = String::new();

                println!("\nWhich employee?");

                io::stdin()
                    .read_line(&mut employee)
                    .expect("Failed to read the employee name");

                let employee = employee.trim().to_lowercase();
                department_entry.push(employee);
                println!("vect = {:?}", departments)
            }
            "l" => {
                println!("Which department?");

                let mut department = String::new();

                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read department");

                let department = department.trim().to_lowercase();
                let department_employees = departments.get(&department);
                println!(
                    "Employees in {}: {}",
                    department,
                    department_employees.unwrap().join(", ")
                );
            }
            "s" => {
                println!("List of employees by departments");
                for (department, employees) in departments.iter() {
                    println!("{} department: {}", department, employees.join(", "));
                }
            }
            _ => (),
        }
    }
}
