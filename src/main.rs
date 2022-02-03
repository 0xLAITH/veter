use std::env;
use std::fs;
use regex::Regex;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let args: Vec<String> = env::args().collect();

    let all_templates = fs::read_to_string("templates.txt")
        .expect("Something went wrong reading the templates.txt file");
    let all_templates: Vec<_> = get_templates(&all_templates);

    let template = &args[1];
    let template = select_template(&template, &all_templates).to_string();
    let mut template = template.lines().skip(1).collect::<Vec<&str>>().join("\n");

    //let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let input = ctx.get_contents().unwrap();

    let input = input.split(" - ").collect();

    let result = modify_template(&mut template, &input);

    println!("{}", result);

    //ctx.set_contents(result.to_owned()).unwrap();
}


fn get_templates(file_string: &String) -> Vec<& str> {
    let split = file_string.split("==========\n");
    let all_templates: Vec<_> = split.collect();
    all_templates
}

fn select_template<'a>(template: &String, all_templates: &'a Vec<& str>) -> &'a str {
    for temp in all_templates {
        if let Some(fline) = temp.lines().nth(0){
            if template.as_str().eq(fline) {
                return &temp
            }
        }
    }
    return "Failed."
}

fn modify_template<'a>(template: &'a mut String, input: &'a Vec<& str>) -> String {
    let token_regex = Regex::new(r"\{(\d+)\}").expect("Invalid regex");
    let mut result = template.clone();
    for cap in token_regex.captures_iter(template) {
        //println!("yo: {} and {:?}", cap[1].parse::<i8>().unwrap(), input[cap[1].parse::<usize>().unwrap()]);
        result = result.replace(&cap[0], &input[cap[1].parse::<usize>().unwrap()]);
    }
    return result
}

