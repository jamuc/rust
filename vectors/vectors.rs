enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is still {}", third),
        None => println!("There was no third element"),
    }

    match v.get(5) {
        Some(fith) => println!("This is the fith element {}", fith),
        None => println!("There was no fith elemet"),
    }

    let mut v1: Vec<i32> = Vec::new();

    v1.push(2);
    println!("The Vector now has an element {}", &v1[0]);

    // you can iterate over vectors
    let v2 = vec![10, 20, 30];
    for element in &v2 {
        println!("The element is {}", element);
    }

    // change each element of a vector
    let mut v3 = vec![25, 35, 45];

    for element in &mut v3 {
        *element += 5;
    }

    for element in &v3 {
        println!("The new elements are {}", element);
    }

    let v4 = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(20.5),
        SpreadsheetCell::Text(String::from("This would hold the text")),
    ];

    for element in &v4 {
        match element {
            SpreadsheetCell::Int(integer) => println!("The integer value is {}", integer),
            SpreadsheetCell::Float(float) => println!("The float value is {}", float),
            SpreadsheetCell::Text(text) => println!("The text value is {}", text),
        }
    }
}
