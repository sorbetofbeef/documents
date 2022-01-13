fn main() {
    let mut people = HashMap::new();
    people.insert("Susan", 21);
    people.insert("Ed", 13);
    people.insert("Will", 14);
    people.insert("Cathy", 22);
    people.remove("Susan");
    
    match people.get("Ed") {
      Some(age) => println!("age = {:?}", age),
      None => println!("not found"),
    }
    
    for (person, age) in people.iter() {
      println!("person = {:?}, age = {:?}", person, age)
    }
    
    for person in people.keys() {
      println!("person = {:?}", person)
    }
    
    for age in people.values() {
      println!(" age = {:?}", age)
    }
}
