use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || (args.len() == 2 && args[1].contains("-h")) {
        println!(" JSON detector which returns TRUE or FALSE \nUsage:\n json-parser <file-path>");
        return;
    }

    if let Ok(file_content) = fs::read_to_string(&args[1]) {
        let mut json: Vec<char> = Vec::new();

        for c in file_content.chars() {
            json.push(c);
        }

        let mut pos = 0 as usize;
        if parse_object(&json, &mut pos) {
            println!("TRUE");
        } else {
            println!("FALSE");
        }
    } else {
        println!("File not found.");
    }
}

fn parse_object(json: &Vec<char>, pos: &mut usize) -> bool {
    escape_whitespace(json, pos);
    let tmp_pos = *pos;
    if parse_char(json, pos, '{') {
        let mut tmp_pos0 = *pos;
        while parse_pair(json, pos) && parse_char(json, pos, ',') {
            tmp_pos0 = *pos;
        }
        *pos = tmp_pos0;

        if parse_pair(json, pos) && parse_char(json, pos, '}') {
            return true;
        }
    }

    *pos = tmp_pos;
    false
}

fn parse_pair(json: &Vec<char>, pos: &mut usize) -> bool {
    escape_whitespace(json, pos);
    let tmp_pos = *pos;

    if parse_stringlit(json, pos) && parse_char(json, pos, ':') && parse_value(json, pos) {
        return true;
    }

    *pos = tmp_pos;
    false
}

fn parse_value(json: &Vec<char>, pos: &mut usize) -> bool {
    escape_whitespace(json, pos);
    let tmp_pos = *pos;
    if parse_object(json, pos)
        || parse_array(json, pos)
        || parse_stringlit(json, pos)
        || parse_num(json, pos)
    {
        return true;
    }
    *pos = tmp_pos;
    false
}

fn parse_stringlit(json: &Vec<char>, pos: &mut usize) -> bool {
    escape_whitespace(json, pos);
    if json[*pos] == '"' {
        for i in *pos + 1..json.len() {
            if json[i] == '"' {
                *pos = i + 1;
                return true;
            }
        }
    }

    return false;
}

fn parse_array(json: &Vec<char>, pos: &mut usize) -> bool {
    escape_whitespace(json, pos);
    let tmp_pos = *pos;

    if parse_char(json, pos, '[') {
        let mut tmp_pos0 = *pos;
        while parse_value(json, pos) && parse_char(json, pos, ',') {
            tmp_pos0 = *pos;
        }
        *pos = tmp_pos0;

        if parse_value(json, pos) && parse_char(json, pos, ']') {
            return true;
        }
    }

    *pos = tmp_pos;
    false
}

fn parse_num(json: &Vec<char>, pos: &mut usize) -> bool {
    escape_whitespace(json, pos);
    for i in *pos..json.len() {
        if json[i].is_digit(10) {
            continue;
        } else if (json[i].is_whitespace() || json[i] == ']' || json[i] == '}' || json[i] == ',')
            && i != *pos
        {
            *pos = i;
            return true;
        } else {
            return false;
        }
    }
    return false;
}

fn parse_char(json: &Vec<char>, pos: &mut usize, ch: char) -> bool {
    escape_whitespace(json, pos);
    if json[*pos] == ch {
        *pos += 1;
        return true;
    }
    return false;
}

fn escape_whitespace(json: &Vec<char>, pos: &mut usize) {
    for i in *pos..json.len() {
        if json[i] != ' ' && json[i] != '\n' {
            *pos = i;
            break;
        }
    }
    return;
}
