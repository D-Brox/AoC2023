pub fn solution1(input:Vec<String>)->u32{
    let mut output:Vec<u32> = Vec::new();
    for line in input{
        let chars: Vec<char> = line.chars()
            .filter(|d| d.is_numeric())
            .collect();
        let first = chars.first().unwrap();
        let last = if let Some(last) = chars.last() {
            last
        }
        else {
            first
        };
        let callibration:u32 = format!("{}{}",first,last).parse().unwrap();// chars.last()
        output.push(callibration);
    }
    
    output.iter().sum()

}

fn replace_numbers(input: String) -> String{
    input.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

pub fn solution2(input:Vec<String>)->u32{
    let mut parsed = Vec::new();
    
    for line in input {
        parsed.push(replace_numbers(line));
    }

    solution1(parsed)
}