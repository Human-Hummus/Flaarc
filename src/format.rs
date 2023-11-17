use crate::Document;
use crate::*;
use std::process::Command;
use std::io::Write;

const OUTPUT_DIRECTORY:&str = "/lib/flaarc/outputs/"; //directory to search for executables to use to make the output


fn run_output(vars: &Vec<Vec<String>>, ir: &String, output:&String) -> String{
    let mut arg1 = String::new();
    for var in vars{
        let mut toadd:String = String::from(var[0].clone()) + ":";
        for charecter in var[1].chars().collect::<Vec<char>>(){
            let ch_as_string:String = String::from(charecter);
            toadd+=match charecter{
                ';' => "\\;",
                '\\' => "\\\\",
                ':' => "\\:",
                _ => ch_as_string.as_str()
            }
        }
        toadd.push(';');
        arg1+=&toadd;
    }
    return String::from_utf8_lossy(&Command::new(OUTPUT_DIRECTORY.to_owned() + output)
                                   .arg(arg1)
                                   .arg(ir)
                                   .output()
                                   .unwrap_or_else(|_error|{
                                        fatal!(format!("fatal error: unable to execute {output}; the output."))
                                   }).stdout).to_string();
}

macro_rules! flip_bool{
    ($bol:expr) => {
        if $bol == true{
            $bol = false;
        }
        else{
            $bol = true;
        }
    }
}

pub fn get_outfname(document: &Document, filename: &String) -> usize{
    let mut x = 0;
    while x < document.files.len(){
        if filename == &document.files[x].filename{
            return x;
        }
        x+=1;
    }
    return 0; // upon failure, return the index page.
}

fn lines_to_pos(text: &Vec<char>, pos: usize) -> usize{
    let mut line_number = 1;
    let mut x = 0;
    while x < text.len() && x != pos{
        if text[x] == '\n'{
            line_number+=1;
        }
        x+=1;
    }
    return line_number;
}

#[derive(PartialEq)]
enum DepthType{
    List,
    URL,
    AlignCenter,
    AlignRight,
    Highlight,
    Table,
    SubScript,
    SquareRoot
}


//parses the... format; generate ir in order to make it easier to parse later to generate HTML, md, etc.
pub fn format_parser(input: &String, doc:&Document) -> String{
    debug!(format!("format parser input: {}", input));
    let mut output = String::new();
    let chars:Vec<char> = input.chars().collect();
    let mut pos = 0;
    
    let mut depthinfo:Vec<DepthType> = vec![];
    let mut is_bold = false;
    let mut is_italic = false;
    let mut is_superscript = false;
    let mut is_table_item = false;
    let mut is_table_row = false;
    let mut is_crossout = false;

    while pos < chars.len(){
        if depthinfo.contains(&DepthType::Table) && chars[pos] != ' ' && chars[pos] != '\t'{
            if !is_table_row{
                flip_bool!(is_table_row);
                output+="\\StartTableRow\\";
            }
            if !is_table_item{
                flip_bool!(is_table_item);
                output+="\\StartTableItem\\"
            }
        }
        if chars[pos] == '_'  && pos < chars.len()-1 && chars[pos+1] == '_'{
            match is_bold{
                true => { output+="\\EndBold\\"; }
                false => { output+="\\StartBold\\"; }
            }
            flip_bool!(is_bold);
            pos+=2;
        }
        else if chars[pos] == '/' &&  pos < chars.len()-1 && chars[pos+1] == '/'{
            match is_italic{
                true => { output+="\\EndItalic\\"; }
                false => { output+="\\StartItalic\\"; }
            }
            flip_bool!(is_italic);
            pos+=2;
        }
        else if chars[pos] == '^'{
            match is_superscript{
                true => { output+="\\EndSuperscript\\"; }
                false => { output+="\\StartSuperscript\\"; }
            }
            flip_bool!(is_superscript);
            pos+=1;
        }
        else if chars[pos] == '-' && pos+1 < chars.len() && chars[pos+1] == '-'{
            match is_crossout{
                true => { output+="\\EndStrike\\"; }
                false => { output+="\\StartStrike\\"; }
            }
            flip_bool!(is_crossout);
            pos+=2;
        }
        else if chars[pos] == '\\'{
            match chars[pos+1]{
                '\\' => { output+="\\\\" }
                '{' => { output+="{" }
                '_' => { output+="_" }
                '/' => { output+="/" }
                '}' => { output+="}" }
                '#' => { output+="#" }
                '-' => { output+="-" }
                _ => { output+="\\"; pos-=1; }
            }
            pos+=2;
        }

        else if chars[pos] == '{'{
            pos+=1;
            let mut command:String = String::new();
            while chars[pos] != ':'{ command.push(chars[pos]); pos+=1 }
            pos+=1;
            while " \n\t".contains(chars[pos]) { pos+=1; }
            
            match command.as_str(){
                "list" => {
                    output+="\\StartList\\";
                    let mut x = pos;
                    while chars[x] != '}' && chars[x] != '\n'{
                        if !" \n\t".contains(chars[x]) { output+="\\StartListItem\\"; break }
                        x+=1;
                    }
                    depthinfo.push(DepthType::List)
                }
                "filelink" => {
                    let mut filename = String::new();
                    let mut linkname = String::new();
                    while chars[pos] != '|'{
                        filename.push(chars[pos]);
                        pos+=1;
                    }
                    pos+=1;
                    while chars[pos] != '}'{
                        linkname.push(chars[pos]);
                        pos+=1;
                    }
                    pos+=1;
                    output+=&format!("\\StartLink\\{}|{}\\EndLink\\", doc.files[get_outfname(doc, &filename)].outfilename, linkname);

                }

                "link" => {
                    let mut link_address = String::new();
                    while !"|}".contains(chars[pos]) {
                        link_address.push(chars[pos]);
                        pos+=1;
                    }
                    output+= &format!("\\StartLink\\{}|", link_address.clone());
        
                    if chars[pos] != '}'{ depthinfo.push(DepthType::URL) }
                    else{ output+= &format!("{}\\EndLink\\", link_address); }
                    pos+=1;
                }

                "center" => {
                    output+="\\StartCenter\\";
                    depthinfo.push(DepthType::AlignCenter)
                }

                "right" => {
                    output+="\\StartRight\\";
                    depthinfo.push(DepthType::AlignRight) 
                }
                "mark" => {
                    output+="\\Startmark\\";
                    depthinfo.push(DepthType::Highlight)
                }
                "table" => {
                    output+="\\StartTable\\";
                    depthinfo.push(DepthType::Table)
                }
                "sub" => {
                    output+="\\StartSubscript\\";
                    depthinfo.push(DepthType::SubScript)
                }
                "sqrt" => {
                    output+="\\StartSquareRoot\\";
                    depthinfo.push(DepthType::SquareRoot)
                }

                _ => { pos+=1; } //just in case
            }


        }
        else if chars[pos] == '}'{
            if depthinfo.len() < 1{
                warn!(format!("warning: unhandled \"}}\" on line {}(pos {})", lines_to_pos(&chars, pos), pos));
                pos+=1;
                continue;
            }
            let terminated = depthinfo.pop().unwrap();
            match terminated{
                DepthType::List         => {output+="\\EndList\\";}     
                DepthType::URL          => {output+="\\EndLink\\";}     
                DepthType::AlignCenter  => {output+="\\EndCenter\\";}   
                DepthType::AlignRight   => {output+="\\EndRight\\";}    
                DepthType::Highlight    => {output+="\\EndMark\\";}     
                DepthType::Table        => {                            
                                            if is_table_item{
                                                flip_bool!(is_table_item);
                                                output+="\\EndTableItem\\"}
                                            if is_table_row{
                                                flip_bool!(is_table_row);
                                                output+="\\EndTableRow\\"}
                                            output+="\\EndTable\\";}
                DepthType::SubScript    => {output+="\\EndSubscript\\"} 
                DepthType::SquareRoot   => {output+="\\EndSquareRoot\\"}

            }
            pos+=1;
        }
        else if chars[pos] == '\n'{
            if depthinfo.contains(&DepthType::List){
                output+="\\EndListItem\\";
                let mut temp_pos = pos+1;
                while temp_pos < chars.len() && chars[temp_pos] != '}'{
                    if !" \n\t".contains(chars[temp_pos]){
                        output+="\\StartListItem\\";
                        break;
                    }
                    temp_pos+=1;
                }
                if !(temp_pos < chars.len()){
                    warn!(format!("Warning: List is unterminated on line {}", lines_to_pos(&chars, pos)));
                }
            }
            else if depthinfo.contains(&DepthType::Table){
                if is_table_item{
                    flip_bool!(is_table_item);
                    output+="\\EndTableItem\\";
                }
                if is_table_row{
                    flip_bool!(is_table_row);
                    output+="\\EndTableRow\\";
                }
            }
            else{
                output+="\n";
            }
            pos+=1;
        }

        else if chars[pos] == '#'{
            let mut action = String::new();
            let mut argument = String::new();
            pos+=1;

            while pos < chars.len() && !" \n".contains(chars[pos]){
                action.push(chars[pos]); pos+=1;
            }
            if action == "break"{output+="\\Break\\"; continue;}
            while pos < chars.len() && chars[pos] != '\n'{ 
                argument.push(chars[pos]); pos+=1;
            }
            if action == "section"{
                output+= &format!("\\Section\\{}\\EndSection\\", argument);
            }
            else if action == "image"{
                output+= &format!("\\StartImage\\{}\\EndImage\\", argument);
            }
            else if action == "quote"{
                output+= &format!("\\StartQuote\\{}\\EndQuote\\", argument);
            }
            else if action == "subsection"{
                output+= &format!("\\SubSection\\{}\\EndSubSection\\", argument);
            }
            pos+=1;
        }

        else if chars[pos] == '|' && depthinfo.contains(&DepthType::Table){
            if is_table_item{
                flip_bool!(is_table_item);
                output+="\\EndTableItem\\";
            }
            pos+=1;
        }

        else{
            output.push(chars[pos]);
            pos+=1;
        }
        
    }
    if is_bold{
        output+="\\EndBold\\";
    }
    if is_italic{
        output+="\\EndItalic\\";
    }
    debug!(format!("parser output: {output}"));
    return output;
}

pub fn output_file(output_type:&String, output_file:&String, ir:&String, vars:&Vec<Vec<String>>){
    std::fs::File::create(output_file)
        .unwrap_or_else(|_|{fatal!(format!("fatal error: unable to open/create file \"{output_file}\""))})
        .write_all(
                run_output(vars, ir, output_type).as_bytes()
            )
        .unwrap_or_else(|_|{fatal!(format!("fatal error: unable to open/create file \"{output_file}\""))});
}
