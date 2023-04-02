use std::fs;
use std::process::Command;
use crate::DocInfo;


pub fn read_file(filename:&String) -> String{
    let data = fs::read_to_string(filename).unwrap_or_else(|_error|{
        println!("Warning: unable to read file with name \"{}\"", filename);
        return "\nFILE SYSTEM ERROR\n".to_string();
    });
    return data.to_string();
}

//count the number of newline chars before the position "pos" 
fn lines_to_pos(vc: &Vec<char>, pos: usize) -> i32{
    let mut count = 0;
    let mut curpos = 0;
    while curpos < pos && curpos < vc.len(){
        if vc[curpos] == '\n'{
            count += 1;
        }
        curpos+=1;
    }
    return count
}

//get the var, give text and position in the text, it'll return the content of the var, and the new
//position. 
fn get_var(text: &String, vars: &Vec<Vec<String>>, mut pos: usize) -> (String, usize){
    pos+=1;
    let chars:Vec<char> = text.chars().collect();
    let mut var_name = String::new();
    while pos < chars.len() && !"\n \t".contains(chars[pos]){ // get name of the var
        var_name.push(chars[pos]);
        pos+=1;
    }
    if chars[pos] != '\n'{pos+=1}
            
    for var in vars{ 
        if var_name == var[0]{
            return (var[1].clone(), pos);
        }
    }
    println!("Warning: variable \"{}\" (on line {}) is unknown.\nMake sure you terminated the variable with whitespace(which will NOT be written to the output),\nand that you didn't put whitespace within or immediately after the variable decleration.", var_name, lines_to_pos(&chars, pos));
    return (String::from("(ERROR; VAR NOT FOUND)"), pos);
}

//run a function.
fn exec_fn(function: &String, text: &String) -> String{
    return String::from_utf8_lossy(&Command::new("/lib/flaarc/".to_owned() + function).arg(text).output().unwrap_or_else(|_error|{
        println!("Warning: function \"{}\" failed to execute", function);
        return Command::new("echo").arg("ERROR: FN FAILED TO EXECUTE").output().unwrap();
    }).stdout).to_string();
}


// This is the Logical Parser. It's a seperate entity from the Formatting Parser due to technical
// reasons. It's job is to take in text and find:
// variable definitions
// variables
// functions &
// include statements
//
// Then process them BEFORE the formatting parser ever sees it; The formatting parser ONLY does
// formatting.
pub fn logical_parser(text: &String, mut vars:Vec<Vec<String>>) -> (String, Vec<Vec<String>>, DocInfo){
    let mut docinf = DocInfo {title: "Title".to_string(), font: "times".to_string()};
    let chars:Vec<char> = text.chars().collect();
    let mut output = String::new();
    let mut pos = 0;

    'mainloop: while pos < chars.len(){
       // print!("{}", chars[pos]);
        if chars[pos] == '\\'{
            match chars[pos+1]{
                '\\' => {output+="\\\\"}
                '#' =>  {output+="\\#"}
                '{' =>  {output+="\\{"}
                '}' =>  {output+="\\}"}
                '$' =>  {output+="$"}
                '_' =>  {output+="\\_"}
                '/' =>  {output+="\\/"}
                '-' => {output+="\\-"}
                _ =>    {output+="\\\\";pos-=1;}
            }
            pos+=2;
        }
        
        else if chars[pos] == '#'{
            let mut action = String::new();
            let mut data = String::new();
            pos+=1;
            if chars[pos] == '#'{
                //we know this is a note; skip line
                while pos < chars.len() && chars[pos] != '\n'{ pos+=1 }
                continue;
            }
            while !" \t".contains(chars[pos]) { action.push(chars[pos]); pos+=1 }   // find the action
            while " \t".contains(chars[pos]) { pos+=1 }                             //skip whitespace.
            while chars[pos] != '\n'{ data.push(chars[pos]); pos+=1 }               // get the data
            
            match action.as_str(){ 
                "define" | "set" | "let" => {
                    //This is a variable.
                    let variable_def_chars:Vec<char> = data.chars().collect();
                    let mut variable_name = String::new();
                    let mut variable_content = String::new();
                    let mut x = 0;
                
                    while x < variable_def_chars.len() && !"\t ".contains(variable_def_chars[x]){ //get the name of the var.
                        variable_name.push(variable_def_chars[x]); x+=1
                    }
                    
                    while x < variable_def_chars.len() && " \t\n".contains(variable_def_chars[x]) {x+=1;} //skip whitespace; find the first char of interest.
                
                    for chr in x..variable_def_chars.len(){ variable_content.push(variable_def_chars[chr]) } // read the contents of the var.
                    let tmp = logical_parser(&variable_content, vars);
                    variable_content = tmp.0;
                    vars = tmp.1;


                    //making this part a loop so that instead of exiting the program, we can break out
                    //of the loop.
                    if variable_name.len() < 1 || variable_content.len() < 1{
                        println!("Warning! illegal variable definition on line {}", lines_to_pos(&chars, pos));
                        output+="\nILLEGAL VARIABLE DEFINITION HERE\n";
                        continue;
                    }

                    //find if (and where) the var is in the vars list.
                    for var_number in 0..vars.len(){
                        if vars[var_number][0] == variable_name{
                            vars[var_number][1] = variable_content; continue 'mainloop;
                        }
                    }
                    vars.push(vec![variable_name, variable_content]); //this runs if the var WASN'T found.
                }

                "include" | "import" | "use" => {
                    //HOLY SHIT!!!!!!1!!!!!!11!! IT'S FUCKING RECURSIVVVVE!!!!!!
                    let tmp = logical_parser(&read_file(&data), vars);
                    output+=&tmp.0;
                    vars = tmp.1;
                }

                "title" => { docinf.title = data }
                "setfont" => { docinf.font = data }

                "section" | "image" | "quote" => { // SKIP THESE; leave them to the format parser
                    output+=&("#".to_string() + &(action.to_string() + &(" ".to_string() + &(data + "\n"))));
                }
                _ => {
                    println!("Warning illegal hash on line {}, with hash's name set to: {}", lines_to_pos(&chars, pos), &action);
                    output+="(ILLEGAL HASH FUNCTION)\n";
                    }
            }
        pos+=1;
        }
        else if chars[pos] == '$'{
            let tmp = get_var(&text, &vars, pos);
            output += &tmp.0;
            pos = tmp.1;
        }
        else if chars[pos] == '{'{
            let prevpos = pos;

            pos+=1;
            let mut function = String::new();
            let mut input = String::new();
            let mut depth = 1; 

            while chars[pos] != ':' && chars[pos] != '}'{ // find what the function //is//
                function+=&chars[pos].to_string();
                pos+=1;
            }
            pos +=1;
            if function == "sub" || function == "center" || function == "right" || function == "list" || function == "link" || function == "mark" || function == "table"{
                output+="{";
                pos = prevpos+1;
            }
            else{
                loop{
                    if depth < 1{break;}

                    if chars[pos] == '\\'{
                        if chars[pos+1] == '\\'{
                            input+="\\\\";
                            pos+=2;
                        }
                        else if chars[pos+1] == '{'{ 
                            input+="\\{";
                            pos+=2;
                        }
                        else if chars[pos+1] == '}'{ 
                            input+="\\}";
                            pos+=2;
                        }
                        else{
                            input.push(chars[pos]);
                            pos+=1;
                        }
                    }
                    else if chars[pos] == '{'{
                        depth+=1;
                        pos+=1;
                    }
                    else if chars[pos] == '}'{
                        depth-=1;
                        pos+=1;
                    }
                    else{
                        input.push(chars[pos]);
                        pos+=1;
                    }
                }
                let parsed_input = logical_parser(&input, vars);
                vars = parsed_input.1;

                let executed = exec_fn(&function, &parsed_input.0);
                let parsed_exec = logical_parser(&executed, vars);
                vars = parsed_exec.1;
                output+=&parsed_exec.0;
            }
        }
        else if chars[pos] == '#'{
            output+="\\#";
            pos+=1;
        }
        else{
            output+=&chars[pos].to_string();
            pos+=1;
        }
    }

    return (output, vars, docinf);
}
