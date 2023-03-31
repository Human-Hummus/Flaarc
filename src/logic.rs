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
    let output;
    let mut var_name = String::new();
    while pos < chars.len() && chars[pos] != '\n' && chars[pos] != ' ' && chars[pos] != '\t'{
        var_name+=&chars[pos].to_string();
        pos+=1;
    }
    let mut x = 0;
            
    while x < vars.len(){
        if var_name == vars[x][0]{
            break;
        }
        x+=1;
    }
    if x < vars.len(){output = vars[x][1].clone();}
    else{
        println!("Warning: variable \"{}\" (on line {}) is unknown.\nMake sure you terminated the variable with whitespace(which will NOT be written to the output),\nand that you didn't put whitespace within or immediately after the variable decleration.", var_name, lines_to_pos(&chars, pos));
        return (String::from("(ERROR; VAR NOT FOUND)"), pos);
    }
    if chars[pos] != '\n'{
        pos+=1;
    }
    return (output, pos);
}

//run a function.
fn exec_fn(function: &String, text: &String) -> String{
    let toret;
    println!("fn name: {}", function);
    toret = String::from_utf8_lossy(&Command::new("/lib/flaarc/".to_owned() + function).arg(text).output().unwrap_or_else(|_error|{
        println!("Warning: function \"{}\" failed to execute", function);
        return Command::new("echo").arg("ERROR: FN FAILED TO EXECUTE").output().unwrap();
    }).stdout).to_string();
    return toret.to_string();
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

    while pos < chars.len(){
        println!("{}", chars[pos]);
        if chars[pos] == '\\'{
            match chars[pos+1]{
                '\\' => {output+="\\\\"}
                '#' =>  {output+="\\#"}
                '{' =>  {output+="\\{"}
                '}' =>  {output+="\\}"}
                '$' =>  {output+="$"}
                '_' =>  {output+="\\_"}
                '/' =>  {output+="\\/"}
                _ =>    {output+="\\\\";pos-=1;}
            }
            pos+=2;
        }
        
        else if chars[pos] == '#'{
            let mut todo:String = String::new();
            let mut run_on:String = String::new();
            pos+=1;
            if chars[pos] == '#'{
                //we know this is a note.
                while pos < chars.len() && chars[pos] != '\n'{
                    pos+=1;
                }
            }
            else{
                while chars[pos] != ' ' && chars[pos] != '\t'{todo+=&chars[pos].to_string();pos+=1;}
                while chars[pos] == ' ' || chars[pos] == '\t'{pos+=1;}
                while chars[pos] != '\n'{run_on+=&chars[pos].to_string();pos+=1;}
            
                if todo == "define" || todo == "set" || todo == "let"{
                    //This is a variable.
                    let variable_def_chars:Vec<char> = run_on.chars().collect();
                    let mut variable_name = String::new();
                    let mut variable_content = String::new();
                    let mut x = 0;
                
                    while x < variable_def_chars.len() && variable_def_chars[x] != ' ' && variable_def_chars[x] != '\t' && variable_def_chars[x] != '\n'{ //get the name of the var.
                        variable_name+=&variable_def_chars[x].to_string();
                        x+=1;
                    }
                    
                    while x < variable_def_chars.len() && (variable_def_chars[x] == ' ' || variable_def_chars[x] == '\t' || variable_def_chars[x] == '\n'){x+=1;} //skip whitespace; find the first char of interest.
                
                    while x<variable_def_chars.len(){ // read the content of the var.
                        variable_content+=&variable_def_chars[x].to_string();
                        x+=1;
                    }
                    let tmp = logical_parser(&variable_content, vars);
                    variable_content = tmp.0;
                    vars = tmp.1;

                    //making this part a loop so that instead of exiting the program, we can break out
                    //of the loop.
                    loop {
                        if variable_name.len() < 1{
                            println!("Warning! illegal variable definition on line {}", lines_to_pos(&chars, pos));
                            output+="\nILLEGAL VARIABLE DEFINITION HERE\n";
                            break;
                        }
                        if variable_content.len() < 1{
                            println!("Warning! Variable \"{}\" on line {} didn't have a proper definition of it's contents", variable_name, lines_to_pos(&chars, pos));
                            output+="\nIMPROPER VARIABLE DEFINITION\n"
                        }
                
                        let mut var_pos = 10000000000000;
                        x = 0;
    
                        //find if (and where) the var is in the vars list.
                        while x < vars.len(){
                            if vars[x][0] == variable_name{
                                var_pos = x;
                                break;
                            }
                            x+=1;
                        }
                        if var_pos != 10000000000000{vars[var_pos][1] = variable_content;}
                        else{vars.push([variable_name, variable_content].to_vec());}
                        break;
                    }
                }
                else if todo == "include" || todo == "import" || todo == "use"{
                    //HOLY SHIT!!!!!!1!!!!!!11!! IT'S FUCKING RECURSIVVVVE!!!!!!
                    let tmp = logical_parser(&read_file(&run_on), vars);
                    output+=&tmp.0;
                    vars = tmp.1;
                }
                else if todo == "title"{
                    docinf.title = run_on;
                }
                else if todo == "setfont"{
                    docinf.font = run_on;
                }
                else if todo == "section"{
                    output+=&("#".to_string() + &(todo.to_string() + &(" ".to_string() + &(run_on + "\n"))));
                }
                else if todo == "image"{
                    output+=&("#".to_string() + &(todo.to_string() + &(" ".to_string() + &(run_on + "\n"))));
                }
                else{
                    println!("Warning illegal hash on line {}, with hash's name set to: {}", lines_to_pos(&chars, pos), &todo);
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
            if function == "center" || function == "right" || function == "list" || function == "link" || function == "mark" || function == "table"{
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
                        input += &chars[pos].to_string();
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
