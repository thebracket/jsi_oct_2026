struct Name;
struct Address;
struct Salary;

struct Employee {
    name: Option<Name>,
    address: Option<Address>,
    salary: Option<Salary>,
}

fn main() {
    let e = Employee {
        name: Some(Name),
        address: Some(Address),
        salary: Some(Salary),
    };
    println!("Employee: named = {}", e.name.is_some());
}
