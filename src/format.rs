use std::fs;
use crate::DocInfo;

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


//parses the... format; generate IR in order to make it easier to parse later to generate HTML, md, etc.
pub fn format_parser(input: &String) -> String{
    let mut output = String::new();
    let chars:Vec<char> = input.chars().collect();
    let mut pos = 0;
    
    let mut depthinfo:Vec<char> = vec![];
    let mut is_bold = false;
    let mut is_italic = false;
    let mut is_superscript = false;
    let mut is_paragraph = false;
    let mut is_table_item = false;
    let mut is_table_row = false;
    let mut is_crossout = false;

    while pos < chars.len(){
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
                '\\' => { output+="\\\\"; pos+=2; }
                '{' => { output+="{"; pos+=2; }
                '_' => { output+="_"; pos+=2; }
                '/' => { output+="/"; pos+=2; }
                '}' => { output+="}"; pos+=2; }
                '#' => { output+="#";  pos+=2; }
                '-' => { output+="-"; pos+=2; }
                _ => { output+="\\"; pos+=1; }
            }
        }

        else if chars[pos] == '{'{
            pos+=1;
            let mut command:String = String::new();
            while chars[pos] != ':'{ command+=&chars[pos].to_string(); pos+=1; }
            pos+=1;
            while chars[pos] == ' ' || chars[pos] == '\n' || chars[pos] == '\t'{ pos+=1; }
            
            match command.as_str(){
                "list" => {
                    output+="\\StartList\\";
                    let mut new_item_follows = false;
                    let mut x = pos;
                    while chars[x] != '}' && chars[x] != '\n'{
                        if chars[x] != ' ' && chars[x] != '\n' && chars[x] != '\t'{ new_item_follows = true; break; }
                        x+=1;
                    }
                    if new_item_follows { output+="\\StartListItem\\"; }
                    depthinfo.push('l');//l is for link
                }

                "link" => {
                    let mut link_address = String::new();
                    while chars[pos] != '|' && chars[pos] != '}'{
                        link_address+=&chars[pos].to_string();
                        pos+=1;
                    }
                    output+=&("\\StartLink:".to_owned() + &(link_address.clone() + "\\"));
        
                    if chars[pos] != '}'{ depthinfo.push('u'); }//U is for Url.
                    else{ output+=&(link_address + "\\EndLink\\"); }
                    pos+=1;
                }

                "center" => {
                    output+="\\StartCenter\\";
                    depthinfo.push('c') //c for center
                }

                "right" => {
                    output+="\\StartRight\\";
                    depthinfo.push('r'); //r for right align.
                }
                "mark" => {
                    output+="\\Startmark\\";
                    depthinfo.push('h'); //h is for highlight.
                }
                "table" => {
                    output+="\\StartTable\\";
                    depthinfo.push('t'); //t for table
                }
                "sub" => {
                    output+="\\StartSubscript\\";
                    depthinfo.push('s'); //s for subscript
                }

                _ => { pos+=1; } //oof
            }


        }
        else if chars[pos] == '}'{
            let terminated:char = depthinfo.pop().unwrap();
            match terminated{
                'l' => {output+="\\EndList\\";}     //list
                'u' => {output+="\\EndLink\\";}     //urls
                'c' => {output+="\\EndCenter\\";}   //center
                'r' => {output+="\\EndRight\\";}    //right align
                'h' => {output+="\\EndMark\\";}     //highlight
                't' => {                            //table
                    if is_table_item{
                        flip_bool!(is_table_item);
                        output+="\\EndTableItem\\"}
                    if is_table_row{
                        flip_bool!(is_table_row);
                        output+="\\EndTableRow\\"}
                    output+="\\EndTable\\";}
                's' => {output+="\\EndSubscript\\"} //subscript   

                _ => {}//somethings wrong
            }
            pos+=1;
        }
        else if chars[pos] == '\n'{
            if depthinfo.contains(&'l'){
                output+="\\EndListItem\\";
                let mut new_item_follows = false;
                let mut temp_pos = pos+1;
                while chars[temp_pos] != '}'{
                    if chars[temp_pos] != ' ' && chars[temp_pos] != '\n' && chars[temp_pos] != '\t'{
                        new_item_follows = true;
                        break;
                    }
                    temp_pos+=1;
                }
                if new_item_follows{
                    output+="\\StartListItem\\";
                }

                pos+=1;
            }
            else if depthinfo.contains(&'t'){
                if is_table_item{
                    flip_bool!(is_table_item);
                    output+="\\EndTableItem\\";
                }
                if is_table_row{
                    flip_bool!(is_table_row);
                    output+="\\EndTableRow\\";
                }
                pos+=1;
            }

            else{
                if pos > 0 && chars[pos-1] == '\n' && is_paragraph{
                    is_paragraph=false;output+="\\EndParagraph\\";
                }
                else{output+="\n";}
                pos+=1;
            }
        }

        else if chars[pos] == '#'{
            let mut action = String::new();
            let mut argument = String::new();
            pos+=1;

            while pos < chars.len() && chars[pos] != ' ' && chars[pos] != '\n'{
                action+=&chars[pos].to_string(); pos+=1;
            }
            while pos < chars.len() && chars[pos] != '\n'{ 
                argument+=&chars[pos].to_string(); pos+=1;
            }
            if action == "section"{
                output+=&("\\Section\\".to_string() + &(argument + "\\EndSection\\"));
            }
            else if action == "image"{
                output+=&("\\StartImage\\".to_string() + &(argument + "\\EndImage\\"));
            }
            else if action == "quote"{
                output+=&("\\StartQuote\\".to_string() + &(argument + "\\EndQuote\\"));
            }
            pos+=1;
        }

        else if chars[pos] == '\t' && (pos < 1 || chars[pos-1] == '\n'){
            if is_paragraph{
                output+="\\EndParagraph\\"
            }
            output+="\\StartParagraph\\";
            is_paragraph = true;
            pos+=1;
        }
        else if chars[pos] == '|' && depthinfo.contains(&'t'){
            if is_table_item{
                flip_bool!(is_table_item);
                output+="\\EndTableItem\\";
            }
            pos+=1;
        }

        else{
            if depthinfo.contains(&'t') && chars[pos] != ' ' && chars[pos] != '\t'{
                if !is_table_row{
                    flip_bool!(is_table_row);
                    output+="\\StartTableRow\\";
                }
                if !is_table_item{
                    flip_bool!(is_table_item);
                    output+="\\StartTableItem\\"
                }
                
            }
            output+=&chars[pos].to_string();
            pos+=1;
        }
        
    }
    if is_paragraph{
        output+="\\EndParagraph\\";
    }
    if is_bold{
        output+="\\EndBold\\";
    }
    if is_italic{
        output+="\\EndItalic\\";
    }
    return output;
}




pub fn markdown_parser(text: &String, output_file: &String, info: DocInfo){
    let mut output = "# ".to_string() + &info.title;
    let mut pos = 0;
    let chars:Vec<char> = text.chars().collect();
    let mut current_link = String::new();
    let mut list_depth = 0;
    let mut table_row_number = 0;
    let mut row_items_number = 0;
    
    while pos < chars.len() {
        //println!("{}, {}", chars[pos], pos);
        if chars[pos] == '\\'{
            if chars[pos+1] == '\\'{
                output+="\\\\";
                pos+=2
            }
            else{
                let mut action = String::new();
                pos+=1;
                while pos < chars.len() && chars[pos] != '\\' && chars[pos] != ':'{
                    action+=&chars[pos].to_string();
                    pos+=1;
                }
                pos+=1;
                match action.as_str(){
                    "StartBold" => {output+="**" }
                    "EndBold" => {output+="**" }
                    "StartItalic" => { output+="*" }
                    "EndItalic" => { output+="*" }
                    "StartList" => { list_depth+=1 }
                    "EndList" => { list_depth-=1 }
                    "StartListItem" => {
                        output+="\n";
                        for _ in 1..list_depth{
                            output+="\t";
                        }
                        output +="- ";
                    }
                    "EndListItem" => {} // do nothing
                    "StartParagraph" => {} // do nothing
                    "EndParagraph" => {} //do nothing
                    "StartLink" => {
                        current_link = String::new();
                        while chars[pos] != '\\'{
                            current_link+=&chars[pos].to_string();pos+=1;
                        }
                        pos+=1;
                        output+="[";
                    }
                    "EndLink" => { output+=&("](".to_owned() + &(current_link.clone() + ")")) }
                    "Section" => { output+="## " }
                    "EndSection" => { output+="\n" }
                    "StartImage" => { output+="![image](" }
                    "EndImage" => { output+=")" }
                    "StartRight" => { output+="<div style=\"text-align: right\">" }
                    "StartCenter" => { output+="<div style=\"text-align: center\">" }
                    "EndRight" => { output+="</div>" }
                    "EndCenter" => { output+="</div>" }
                    "StartTable" => {table_row_number=0;} //do nothing
                    "EndTable" => {} //do nothing
                    "StartTableRow" => {
                        if table_row_number == 1{
                            output+="\n|";
                            while 0 < row_items_number{
                                row_items_number-=1;
                                output+="---|";
                            }
                        }
                        output+="\n|";
                        table_row_number+=1;
                        row_items_number=0;
                    }
                    "EndTableRow" => {} //do nothing
                    "StartTableItem" => {} //do nothing
                    "EndTableItem" => {
                        output+="|";
                        row_items_number+=1;
                    }
                    "Startmark" => { output+="<mark>" }
                    "EndMark" => { output+="</mark>" }
                    "StartSuperscript" => { output+="<sup>" }
                    "EndSuperscript" => { output+="</sup>" }
                    "StartSuperscript" => { output+="<sub>" }
                    "EndSubscript" => { output+="</sub>" }
                    "StartQuote" => { output+="\n>" }
                    "EndQuote" => { output+="\n" }
                    "StartStrike" => { output+="~~" }
                    "EndStrike" => { output+="~~" }
                    _ => {}
                }

            }
        }
        else if "_*#<>".contains(chars[pos]){match chars[pos]{
            '_' => { output+="\\_" }
            '*' => { output+="\\*" }
            '#' => { output+="\\#" }
            '<' => { output+="\\<" }
            '>' => { output+="\\>" }
            _ => {}
        }pos+=1}
        else{
            output += &chars[pos].to_string();
            pos+=1;
        }

    }

    fs::write(output_file, output).expect("error writing file");
}


pub fn html_parser(text: &String, output_file: &String, info: DocInfo){
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
                match action.as_str(){
                    "StartBold" => { output+="<b>" }
                    "EndBold" => { output+="</b>" }
                    "StartItalic" => { output+="<em>" }
                    "EndItalic" => { output+="</em>" }
                    "StartList" => { output+="<ul>" }
                    "EndList" => { output+="</ul>" }
                    "StartListItem" => { output+="<li>" }
                    "EndListItem" => { output+="</li>" }
                    "StartParagraph" => { output+=&("<p style=\"font-family:\'".to_owned() + &(info.font.clone() + "\'\">")) }
                    "EndParagraph" => { output+="</p>" }
                    "StartLink" => {
                        let mut link_to = String::new();
                        while chars[pos] != '\\'{
                            link_to+=&chars[pos].to_string();
                            pos+=1;
                        }
                        pos+=1;
                        output+=&("<a href=\"".to_owned() + &(link_to + "\">"));
                    }
                    "EndLink" => { output+="</a>" }
                    "Section" => { output+="<h2>" }
                    "Startmark" => { output+="<mark>" }
                    "EndSection" => { output+="</h2>" }
                    "StartImage" => { output+="<img src=\"" }
                    "EndImage" => { output+="\">" }
                    "StartRight" => { output+="<div style=\"text-align: right\">" }
                    "StartCenter" => { output+="<div style=\"text-align: center\">"}
                    "EndRight" => { output+="</div>" }
                    "EndCenter" => { output+="</div>" }
                    "EndMark" => { output+="</mark>" }
                    "StartTable" => { output+="<table>" }
                    "EndTable" => { output+="</table>" }
                    "StartTableItem" => { output+="<th>" }
                    "EndTableItem" => { output+="</th>" }
                    "StartTableRow" => { output+="<tr>" }
                    "EndTableRow" => { output+="</tr>" }
                    "StartSuperscript" => { output+="<sup>" }
                    "EndSuperscript" => { output+="</sup>" }
                    "StartSuperscript" => { output+="<sub>" }
                    "EndSubscript" => { output+="</sub>" }
                    "StartQuote" => { output+="<blockquote>" }
                    "EndQuote" => { output+="</blockquote>" }
                    "StartStrike" => { output+="<del>" }
                    "EndStrike" => { output+="</del>" }
                    _ => {}
                }
            }
            
        }
        else if "\n<>\"\'&".contains(chars[pos]){match chars[pos]{
            '\n' => { output+="<br>" }
            '<' => { output+="&lt;" }
            '>' => { output+="&gt;" }
            '"' => { output+="&quot;" }
            '\'' => { output+="&apos;" }   
            '&' => { output+="&amp;" }
            _ => {}
        }pos+=1}
        else{
            output+=&chars[pos].to_string();
            pos+=1;
        }
    }
    output+="</p></body></html>";

    fs::write(output_file, output).expect("error writing file");
}

pub fn text_parser(text: &String, output_file: &String, info: DocInfo){
    let mut output = "".to_string();
    output+=&info.title;
    output+="___________________________________________";
    let chars:Vec<char> = text.chars().collect();
    let mut pos = 0;
    let mut indents = 0;

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
                match action.as_str(){
                    "StartBold" => { output+="__" }
                    "EndBold" => { output+="__" }
                    "StartItalic" => { output+="//" }
                    "EndItalic" => { output+="//" }
                    "StartList" => { indents+=1 }
                    "EndList" => { indents-=1 }
                    "StartListItem" => {
                        let mut indents_done = 0;
                        while indents_done < indents{
                            output+="\t";
                            indents_done+=1;
                        }
                        output+="> ";
                    }
                    "EndListItem" => { output+="\n" }
                    "StartParagraph" => { output+="\n\t" }
                    "EndParagraph" => { output+="\n" }
                    "StartLink" => {
                        let mut link = String::new();
                        while chars[pos] != '\\'{
                            link +=&chars[pos].to_string();
                            pos+=1;
                        }
                        pos+=1;
                        output+=&("(LINK: \"".to_string() + &(link + "\" Text: "));
                    }
                    "EndLink" => { output+=")" }
                    "Section" => { output+="=========================\n" }
                    "EndSection" => { output+="\n=========================\n" }
                    "StartRight" => {output+="<div style=\"text-align: right\">"}
                    "EndRight" => {output+="</div>"}
                    "StartCenter" => {output+="<div style=\"text-align: center\">"}
                    "EndCenter" => {output+="</div>"}
                    "Startmark" => {output+="<mark>"}
                    "EndMark" => {output+="</mark>"}
                    _ => {}
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
