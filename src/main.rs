use regex::Regex;
use std::fs;

fn main() {
    // remove_special_characters();
    replace_characters_for_tailwind();
}

fn remove_special_characters() {
    let file = fs::read_to_string("puzzle1.txt").unwrap();
    let re = Regex::new("[^A-Za-z0-9+/=]").unwrap();

    let result = re.replace_all(&file[..], "");
    let result = str::replace(&result, "dataimage/", "");
    let result = str::replace(&result, "jpegbase64", "");

    println!("{}", result);
}

fn replace_characters_for_tailwind() {
    let file = fs::read_to_string("puzzle3.html").unwrap();

    let chars_to_replace = vec![
        ("#", "x"),
        ("$", "s"),
        ("!", "i"),
        ("~", "-"),
        ("|", "1"),
        ("@", "a"),
        ("*", "o"),
        ("&", "7"),
    ];

    let res: Vec<String> = file
        .lines()
        .map(|line| {
            let mut new_line = String::new();
            if line.contains("class") {
                let splited: Vec<&str> = line.trim().split("\"").collect();

                let result: Vec<String> = splited
                    .iter()
                    .enumerate()
                    .map(|(key, string)| {
                        let mut new_string: String = string.to_string();
                        if key == 1 {
                            new_string = string.chars().rev().collect::<String>();
                            for (from, to) in chars_to_replace.iter() {
                                new_string = str::replace(&new_string, from, to);
                            }
                            new_string = String::from("\"")
                                + &new_string
                                + &String::from("\"");
                        }
                        new_string
                    })
                    .collect();
                new_line = result.concat();
            }
            new_line
        })
        .collect();

    let items_to_replace: Vec<Option<String>> = res
        .iter()
        .map(|line| {
            if !line.is_empty() {
                Some(line.clone())
            } else {
                None
            }
        })
        .collect();

    let mut output: Vec<&str> = Vec::new();

    for (index, line) in file.lines().enumerate() {
        let res = match &items_to_replace[index] {
            Some(content) => content,
            None => line,
        };
        output.push(res);
    }

    println!("{}", output.concat());
}
