use std::fs;
use crate::docinfo;

//parses the... format; generate IR in order to make it easier to parse later to generate HTML, md, etc.
pub fn format_parser(input: &String) -> String{
    let mut output = String::new();
    let chars:Vec<char> = input.chars().collect();
    let mut pos = 0;
    
    let mut depthinfo:Vec<char> = vec![];
    
    let mut is_bold = false;
    let mut is_italic = false;
    let mut is_paragraph = false;
    let mut is_link = false;

    while pos < chars.len(){
        if chars[pos] == '_'  && pos < chars.len()-1 && chars[pos+1] == '_'{
            if is_bold{
                output+="\\ENDBOLD\\";
                is_bold = false;
            }
            else{
                output+="\\STARTBOLD\\";
                is_bold = true;
            }
            pos+=2;
        }
        else if chars[pos] == '/' &&  pos < chars.len()-1 && chars[pos+1] == '/'{
            if is_italic{
                output+="\\ENDITALIC\\";
                is_italic = false;
            }
            else{
                output+="\\STARTITALIC\\";
                is_italic = true;
            }
            pos+=2;
        }
        else if chars[pos] == '\\'{
            if chars[pos+1] == '\\'{
                output+="\\\\";
                pos+=2;
            }
            else if chars[pos+1] == '{'{ 
                output+="{";
                pos+=2;
            }
            else if chars[pos+1] == '_'{             
                output+="_"; 
                pos+=2;
            }
            else if chars[pos+1] == '/'{             
                output+="/"; 
                pos+=2;
            }
            else if chars[pos+1] == '}'{ 
                output+="}";  
                pos+=2;
            }
            else{
                output+="\\";
                pos+=1;
            }
        }

        else if chars[pos] == '{'{
            pos+=1;
            let mut command:String = String::new();
            while chars[pos] != ':'{
                command+=&chars[pos].to_string();
                pos+=1;
            }
            pos+=1;
            while chars[pos] == ' ' || chars[pos] == '\n' || chars[pos] == '\t'{
                pos+=1;
            }
            if command == "list"{
                println!("list");
                output+="\\STARTLIST\\";
                let mut new_item = false;
                let mut tmp = pos+1;
                while chars[tmp] != '}' && chars[pos] != '\n'{
                    if chars[tmp] != ' ' && chars[tmp] != '\n' && chars[tmp] != '\t'{
                        new_item = true;
                        break;
                    }
                    tmp+=1;
                }
                if new_item{
                    output+="\\STARTLISTITEM\\";
                }
                depthinfo.push('l');
                pos+=1;
            }
            else if command == "link"{
                let mut linkto = String::new();
                while (chars[pos] != '|' && chars[pos] != '}'){
                    linkto+=&chars[pos].to_string();
                    pos+=1;
                }
                output+=&("\\STARTLINK:".to_owned() + &(linkto.clone() + "\\"));
                //U for Url.
                if chars[pos] != '}'{
                    depthinfo.push('u');
                    pos+=1;
                }
                else{
                    output+=&linkto;
                    output+="\\ENDLINK\\";
                    pos+=1;
                }
            }
        }
        else if chars[pos] == '}'{
            let terminated:char = depthinfo.pop().unwrap();
            if terminated == 'l'{
                output+="\\ENDLIST\\";
            }
            if terminated == 'u'{
                output+="\\ENDLINK\\";
            }
            pos+=1;
        }
        else if chars[pos] == '\n'{
            if depthinfo.contains(&'l'){
                output+="\\ENDLISTITEM\\";
                let mut new_item = false;
                let mut tmp = pos+1;
                while chars[tmp] != '}'{
                    if chars[tmp] != ' ' && chars[tmp] != '\n' && chars[tmp] != '\t'{
                        new_item = true;
                        break;
                    }
                    tmp+=1;
                }
                if new_item{
                    output+="\\STARTLISTITEM\\";
                }

                pos+=1;
            }
            else{
                if pos < 1 || chars[pos-1] == '\n'{if is_paragraph{is_paragraph=false;output+="\\ENDPARAGRAPH\\"}}

                else{output+="\n";}
                pos+=1;
            }


        }
        else if chars[pos] == '\t' && (pos < 1 || chars[pos-1] == '\n'){
            if is_paragraph{
                output+="\\ENDPARAGRAPH\\"
            }
            output+="\\STARTPARAGRAPH\\";
            is_paragraph = true;
            pos+=1;
        }

        else{
            output+=&chars[pos].to_string();
            pos+=1;
        }
        
    }
    if is_paragraph{
        output+="\\ENDPARAGRAPH\\";
    }
    if is_bold{
        output+="\\ENDBOLD\\"
    }
    return output;
}




pub fn markdown_parser(text: &String, output_file: &String, info: docinfo){
    let mut output = String::new();
    output+=&("%".to_owned() + &(info.title.to_owned() + "\n"));
    let chars:Vec<char> = text.chars().collect();
    let mut pos = 0;

    while pos < chars.len(){
        if chars[pos] == '\\'{
            if chars[pos+1] == '\\'{
                output+="\\";
                pos+=2;
            }
            else{
                pos+=1;
                let mut action = String::new();
                while chars[pos] != '\\'{
                    action+=&chars[pos].to_string();
                    pos+=1;
                }
                pos+=1;
                if action == "STARTBOLD" || action == "ENDBOLD"{
                    output+="**";
                }
                else if action == "STARTITALIC" || action == "ENDITALIC"{
                    output+="*";
                }
            }
            
        }
        else if chars[pos] == '*'{
            output+="\\*";
            pos+=1;
        }
        else if chars[pos] == '_'{ 
            output+="\\_";
            pos+=1;
        }
        else if chars[pos] == '='{ 
            output+="\\=";
            pos+=1;
        }
        else if chars[pos] == '-'{ 
            output+="\\-";
            pos+=1;
        }
        else{
            output+=&chars[pos].to_string();
            pos+=1;
        }
        

    }

    fs::write(output_file, output).expect("error writing file");
}


pub fn HTML_parser(text: &String, output_file: &String, info: docinfo){
    let mut output = "<!DOCTYPE html><html>".to_string();
    output+=&("<head><title>".to_owned() + &(info.title.clone() + "</title></head>"));
    output += &("<body><h1>".to_owned() + &(info.title + "</h1>"));
    let chars:Vec<char> = text.chars().collect();
    let mut pos = 0;

    while pos < chars.len(){
        if chars[pos] == '\\'{
            if chars[pos+1] == '\\'{
                output+="\\";
                pos+=2;
            }
            else{
                pos+=1;
                let mut action = String::new();
                while chars[pos] != '\\' && chars[pos] != ':'{
                    action+=&chars[pos].to_string();
                    pos+=1;
                }
                pos+=1;
                if action == "STARTBOLD"{
                    output+="<b>";
                }
                else if action == "ENDBOLD"{
                    output+="</b>";
                }
                else if action == "STARTITALIC"{
                    output+="<em>";
                }
                else if action == "ENDITALIC"{
                    output+="</em>";
                }
                else if action == "STARTLIST"{
                    output+="<ul>";
                }
                else if action == "ENDLIST"{
                    output+="</ul>";
                }
                else if action == "STARTLISTITEM"{
                    output+="<li>";
                }
                else if action == "ENDLISTITEM"{
                    output+="</li>";
                }
                else if action == "STARTPARAGRAPH"{
                    output+=&("<p style=\"font-family:\'".to_owned() + &(info.font.clone() + "\'\"> &nbsp;&nbsp;&nbsp;&nbsp;"));
                }
                else if action == "ENDPARAGRAPH"{
                    output+="</p>";
                }
                else if action == "STARTLINK"{
                    let mut link_to = String::new();
                    while chars[pos] != '\\'{
                        link_to+=&chars[pos].to_string();
                        pos+=1;
                    }
                    pos+=1;
                    output+=&("<a href=\"".to_owned() + &(link_to + "\">"));

                }
                else if action == "ENDLINK"{
                    output+="</a>";
                }
            }
            
        }
        //else if chars[pos] == '*'{
        //    output+="\\*";
        //    pos+=1;
        //}
        else if chars[pos] == '\n'{ 
            output+="<br>";
            pos+=1;
        }
        else{
            output+=&chars[pos].to_string();
            pos+=1;
        }
    }
    output+="</p></body></html>";

    fs::write(output_file, output).expect("error writing file");
}

pub fn text_parser(text: &String, output_file: &String, info: docinfo){
    let mut output = "".to_string();
    output+=&info.title;
    output+="___________________________________________";
    let chars:Vec<char> = text.chars().collect();
    let mut pos = 0;

    while pos < chars.len(){
        if chars[pos] == '\\'{
            if chars[pos+1] == '\\'{
                output+="\\";
                pos+=2;
            }
            else{
                pos+=1;
                let mut action = String::new();
                while chars[pos] != '\\'{
                    action+=&chars[pos].to_string();
                    pos+=1;
                }
                pos+=1;
                if action == "STARTBOLD"{
                    output+="__";
                }
                else if action == "ENDBOLD"{
                    output+="__";
                }
                else if action == "STARTITALIC"{
                    output+="//";
                }
                else if action == "ENDITALIC"{
                    output+="//";
                }
            }
            
        }
        else{
            output+=&chars[pos].to_string();
            pos+=1;
        }
    }
    output+="";

    fs::write(output_file, output).expect("error writing file");
}
