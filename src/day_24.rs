use std::collections::HashMap;

pub fn parse_input(
    input: &str,
) -> (
    HashMap<String, usize>,
    HashMap<String, (String, String, String)>,
) {
    let (values, logic) = input.split_once("\n\n").unwrap();
    let values: Vec<(String, usize)> = values
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(": ").unwrap();
            let value = value.parse::<usize>().unwrap();
            (key.to_string(), value)
        })
        .collect();
    let values = HashMap::from_iter(values);
    let logic: Vec<(String, (String, String, String))> = logic
        .lines()
        .map(|line| {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            let op1 = tokens[0].to_string();
            let op = tokens[1].to_string();
            let op2 = tokens[2].to_string();
            let target = tokens[4].to_string();
            (target, (op1, op, op2))
        })
        .collect();
    let logic: HashMap<String, (String, String, String)> = HashMap::from_iter(logic);
    (values, logic)
}

pub fn solve_part_1(
    input: &(
        HashMap<String, usize>,
        HashMap<String, (String, String, String)>,
    ),
) -> usize {
    let mut values = input.0.clone();
    let mut logic = input.1.clone();
    while logic.len() > 0 {
        println!("pass");
        let mut to_remove = Vec::new();
        for (key, (op1, op, op2)) in logic.iter() {
            if !values.contains_key(op1) || !values.contains_key(op2) {
                continue;
            }
            println!("{} {} {} -> {}", op1, op, op2, key);
            let op1 = values[op1];
            let op2 = values[op2];
            let result = match op.as_str() {
                "AND" => op1 & op2,
                "OR" => op1 | op2,
                "XOR" => op1 ^ op2,
                _ => panic!("Unknown operator"),
            };
            values.insert(key.to_string(), result);
            to_remove.push(key.to_string());
        }
        for key in to_remove {
            logic.remove(&key);
        }
    }
    let mut answer = 0;
    let mut i = 0;
    loop {
        if !values.contains_key(&format!("z{:02}", i)) {
            break;
        }
        let bit = values[&format!("z{:02}", i)];
        answer += bit << i;
        i += 1;
    }
    answer
}

pub fn solve_part_2(
    input: &(
        HashMap<String, usize>,
        HashMap<String, (String, String, String)>,
    ),
) -> usize {
    let operations = solve(45);
    let mut key_to_operation = HashMap::new();
    for (key, op1, op, op2) in &operations {
        key_to_operation.insert(key, (op1, op, op2));
    }
    // Invert the input logic
    let mut values = input.0.clone();
    let mut logic = input.1.clone();
    let mut inverted_logic = HashMap::new();
    for (key, (op1, op, op2)) in logic.iter() {
        inverted_logic.insert(
            (op1.to_string(), op.to_string(), op2.to_string()),
            key.to_string(),
        );
    }
    let mut variable_name_map = HashMap::new();
    for i in 0..45 {
        variable_name_map.insert(format!("x{:02}", i), format!("x{:02}", i));
        variable_name_map.insert(format!("y{:02}", i), format!("y{:02}", i));
        variable_name_map.insert(format!("z{:02}", i), format!("z{:02}", i));
    }
    variable_name_map.insert("z45".to_string(), "z45".to_string());

    for (key, op1, op, op2) in &operations {
        let op1_translated = variable_name_map.get(op1).unwrap();
        let op2_translated = variable_name_map.get(op2).unwrap();
        let key_translated = if let Some(key) =
            inverted_logic.get(&(op1_translated.clone(), op.clone(), op2_translated.clone()))
        {
            key.clone()
        } else if let Some(key) =
            inverted_logic.get(&(op2_translated.clone(), op.clone(), op1_translated.clone()))
        {
            key.clone()
        } else {
            "not_found".to_string()
        };
        if &key_translated == "not_found" {
            println!("Not found:");
            println!("{} {} {} -> {}", op1, op, op2, key);
            println!(
                "{} {} {} -> {}",
                op1_translated, op, op2_translated, key_translated
            );
            break;
        }
        match variable_name_map.get(key) {
            Some(v) => {
                if v != &key_translated {
                    println!("Found conflict");
                    println!("{} = {}", key, key_translated);
                    break;
                }
            }
            None => {}
        }
        println!(
            "{} {} {} -> {}",
            op1_translated, op, op2_translated, key_translated
        );
        println!("{} {} {} -> {}", op1, op, op2, key);
        println!();
        variable_name_map.insert(key.clone(), key_translated);
    }
    println!();
    // for (k, v) in &variable_name_map {
    // println!("{} -> {}", k, v);
    // }

    0
}

fn solve(num_of_bits: usize) -> Vec<(String, String, String, String)> {
    let mut operations = Vec::new();
    operations.push((
        "z00".to_string(),
        "x00".to_string(),
        "XOR".to_string(),
        "y00".to_string(),
    ));
    operations.push((
        "c00".to_string(),
        "x00".to_string(),
        "AND".to_string(),
        "y00".to_string(),
    ));

    for i in 1..num_of_bits {
        let yipxi = format!("x{:02}_p_y{:02}", i, i);
        operations.push((
            yipxi.clone(),
            format!("x{:02}", i),
            "XOR".to_string(),
            format!("y{:02}", i),
        ));
        operations.push((
            format!("z{:02}", i),
            yipxi.clone(),
            "XOR".to_string(),
            format!("c{:02}", i - 1),
        ));
        operations.push((
            format!("c{:02}_a_{}", i - 1, yipxi.clone()),
            format!("c{:02}", i - 1),
            "AND".to_string(),
            yipxi.clone(),
        ));
        operations.push((
            format!("x{:02}_a_y{:02}", i, i),
            format!("x{:02}", i),
            "AND".to_string(),
            format!("y{:02}", i),
        ));
        operations.push((
            format!("c{:02}", i),
            format!("x{:02}_a_y{:02}", i, i),
            "OR".to_string(),
            format!("c{:02}_a_{}", i - 1, yipxi.clone()),
        ));
    }
    let last_operation = operations.pop().unwrap();
    operations.push((
        format!("z{:02}", num_of_bits),
        last_operation.1,
        last_operation.2,
        last_operation.3,
    ));
    operations
}
