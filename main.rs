

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn gen_table(key: Vec<usize>) -> Vec<[char; 26]> {

    let abc: Vec<u8> = (65..=90).collect();

    let mut table: Vec<[char; 26]> = Vec::new();

    for mut i in key {
        let mut col: [char; 26] = ['0'; 26];

        for j in 0..26 {
            if i > 25 {
               i = 0;
            }
            col[i] = abc[j] as char;
            i = i + 1;
        }
        table.push(col);
    }
    return table;
}


fn main() {
    
    println!("Enter Key Phrase: ");

    let mut key = String::new();

    std::io::stdin()
        .read_line(&mut key)
        .expect("Failed To Read In");
        
    
    let offset: Vec<usize> = first_word(&key).to_ascii_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c as usize - 64)
        .collect();
    
    // for i in key_w.chars() {
    //     offset.push(char_to_num(&i));
    // }    

    let table = gen_table(offset); 

    // for i in &table {
        
    //     for j in i {
    //         print!("{} ", j);
    //     }
    //     println!("");
    // }

    println!("Enter Message: ");

    let mut phrase = String::new();

    std::io::stdin()
        .read_line(&mut phrase)
        .expect("Failed To Read In");

    let input: Vec<usize> = phrase.to_ascii_uppercase()
        .chars()
        .map(|c| if c.is_ascii_alphabetic() {c as usize - 65} else{256})
        .collect();
   
    let mut output = String::new();

    for (i, val) in input.iter().enumerate() {
        if *val < 25 {
            output.push(if i % table.len() == 0 {table[table.len() - 1][*val]} else {table[i % table.len()][*val]})
        } 
        else {
            output.push(' ');
        }
    }
    
    print!("{}", output);
}
