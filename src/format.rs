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

    while pos < chars.len(){
        if chars[pos] == '_'  && pos < chars.len()-1 && chars[pos+1] == '_'{
            match is_bold{
                true => { output+="\\ENDBOLD\\"; }
                false => { output+="\\STARTBOLD\\"; }
            }
            flip_bool!(is_bold);
            pos+=2;
        }
        else if chars[pos] == '/' &&  pos < chars.len()-1 && chars[pos+1] == '/'{
            match is_italic{
                true => { output+="\\ENDITALIC\\"; }
                false => { output+="\\STARTITALIC\\"; }
            }
            flip_bool!(is_italic);
            pos+=2;
        }
        else if chars[pos] == '^'{
            match is_superscript{
                true => { output+="\\ENDSUPERSCRIPT\\"; }
                false => { output+="\\STARTSUPERSCRIPT\\"; }
            }
            flip_bool!(is_superscript);
            pos+=1;
        }
        else if chars[pos] == '\\'{
            match chars[pos+1]{
                '\\' => { output+="\\\\"; pos+=2; }
                '{' => { output+="{"; pos+=2; }
                '_' => { output+="_"; pos+=2; }
                '/' => { output+="/"; pos+=2; }
                '}' => { output+="}"; pos+=2; }
                '#' => { output+="#";  pos+=2; }
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
                    output+="\\STARTLIST\\";
                    let mut new_item_follows = false;
                    let mut x = pos;
                    while chars[x] != '}' && chars[x] != '\n'{
                        if chars[x] != ' ' && chars[x] != '\n' && chars[x] != '\t'{ new_item_follows = true; break; }
                        x+=1;
                    }
                    if new_item_follows { output+="\\STARTLISTITEM\\"; }
                    depthinfo.push('l');//l is for link
                }

                "link" => {
                    let mut link_address = String::new();
                    while chars[pos] != '|' && chars[pos] != '}'{
                        link_address+=&chars[pos].to_string();
                        pos+=1;
                    }
                    output+=&("\\STARTLINK:".to_owned() + &(link_address.clone() + "\\"));
        
                    if chars[pos] != '}'{ depthinfo.push('u'); }//U is for Url.
                    else{ output+=&(link_address + "\\ENDLINK\\"); }
                    pos+=1;
                }

                "center" => {
                    output+="\\STARTCENTER\\";
                    depthinfo.push('c') //c for center
                }

                "right" => {
                    output+="\\STARTRIGHT\\";
                    depthinfo.push('r'); //r for right align.
                }
                "mark" => {
                    output+="\\STARTMARK\\";
                    depthinfo.push('h'); //h is for highlight.
                }
                "table" => {
                    output+="\\STARTTABLE\\";
                    depthinfo.push('t'); //t for table
                }
                "sub" => {
                    output+="\\STARTSUBSCRIPT\\";
                    depthinfo.push('s'); //s for subscript
                }

                _ => { pos+=1; } //oof
            }


        }
        else if chars[pos] == '}'{
            let terminated:char = depthinfo.pop().unwrap();
            match terminated{
                'l' => {output+="\\ENDLIST\\";}     //list
                'u' => {output+="\\ENDLINK\\";}     //urls
                'c' => {output+="\\ENDCENTER\\";}   //center
                'r' => {output+="\\ENDRIGHT\\";}    //right align
                'h' => {output+="\\ENDMARK\\";}     //highlight
                't' => {                            //table
                    if is_table_item{
                        flip_bool!(is_table_item);
                        output+="\\ENDTABLEITEM\\"}
                    if is_table_row{
                        flip_bool!(is_table_row);
                        output+="\\ENDTABLEROW\\"}
                    output+="\\ENDTABLE\\";}
                's' => {output+="\\ENDSUBSCRIPT\\"} //subscript   

                _ => {}//somethings wrong
            }
            pos+=1;
        }
        else if chars[pos] == '\n'{
            if depthinfo.contains(&'l'){
                output+="\\ENDLISTITEM\\";
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
                    output+="\\STARTLISTITEM\\";
                }

                pos+=1;
            }
            else if depthinfo.contains(&'t'){
                if is_table_item{
                    flip_bool!(is_table_item);
                    output+="\\ENDTABLEITEM\\";
                }
                if is_table_row{
                    flip_bool!(is_table_row);
                    output+="\\ENDTABLEROW\\";
                }
                pos+=1;
            }

            else{
                if pos > 0 && chars[pos-1] == '\n' && is_paragraph{
                    is_paragraph=false;output+="\\ENDPARAGRAPH\\";
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
                output+=&("\\SECTION\\".to_string() + &(argument + "\\ENDSECTION\\"));
            }
            else if action == "image"{
                output+=&("\\STARTIMAGE\\".to_string() + &(argument + "\\ENDIMAGE\\"));
            }
            pos+=1;
        }

        else if chars[pos] == '\t' && (pos < 1 || chars[pos-1] == '\n'){
            if is_paragraph{
                output+="\\ENDPARAGRAPH\\"
            }
            output+="\\STARTPARAGRAPH\\";
            is_paragraph = true;
            pos+=1;
        }
        else if chars[pos] == '|' && depthinfo.contains(&'t'){
            if is_table_item{
                flip_bool!(is_table_item);
                output+="\\ENDTABLEITEM\\";
            }
            pos+=1;
        }

        else{
            if depthinfo.contains(&'t') && chars[pos] != ' ' && chars[pos] != '\t'{
                if !is_table_row{
                    flip_bool!(is_table_row);
                    output+="\\STARTTABLEROW\\";
                }
                if !is_table_item{
                    flip_bool!(is_table_item);
                    output+="\\STARTTABLEITEM\\"
                }
                
            }
            output+=&chars[pos].to_string();
            pos+=1;
        }
        
    }
    if is_paragraph{
        output+="\\ENDPARAGRAPH\\";
    }
    if is_bold{
        output+="\\ENDBOLD\\";
    }
    if is_italic{
        output+="\\ENDITALIC\\";
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
                if action == "STARTBOLD"{
                    output+="**"
                }
                else if action == "ENDBOLD"{
                    output+="**"
                }
                else if action == "STARTITALIC"{ 
                    output+="*"
                }
                else if action == "ENDITALIC"{ 
                    output+="*"
                }
                else if action == "STARTLIST"{ 
                    list_depth+=1;
                }
                else if action == "ENDLIST"{ 
                    list_depth-=1;
                }
                else if action == "STARTLISTITEM"{
                    output+="\n";
                    for _ in 1..list_depth{
                        output+="\t";
                    }
                    output +="- ";
                }
                else if action == "ENDLISTITEM"{
                    //do nothing.
                }
                else if action == "STARTPARAGRAPH"{
                    output+="";
                }
                else if action == "ENDPARAGRAPH"{
                    //do nothing
                }
                else if action == "STARTLINK"{
                    current_link = String::new();
                    while chars[pos] != '\\'{
                        current_link+=&chars[pos].to_string();pos+=1;
                    }
                    pos+=1;
                    output+="[";
                }
                else if action == "ENDLINK"{
                    output+=&("](".to_owned() + &(current_link.clone() + ")"));
                }
                else if action == "SECTION"{
                    output+="## ";
                }
                else if action == "ENDSECTION"{
                    output+="\n";
                }
                else if action == "STARTIMAGE"{
                    output+="![image](";
                }
                else if action == "ENDIMAGE"{
                    output+=")";
                }
                else if action == "STARTRIGHT"{
                    output+="<div style=\"text-align: right\">";
                }
                else if action == "STARTCENTER"{
                    output+="<div style=\"text-align: center\">";
                }
                else if action == "ENDRIGHT"{
                    output+="</div>"
                }
                else if action == "ENDCENTER"{
                    output+="</div>"
                }
                else if action == "STARTTABLE"{
                    //do nothing
                }
                else if action == "ENDTABLE"{
                    //do nothing
                }
                else if action == "STARTTABLEROW"{
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
                else if action == "ENDTABLEROW"{
                    //do nothing
                }
                else if action == "STARTTABLEITEM"{
                    //do nothing
                }
                else if action == "ENDTABLEITEM"{
                    output+="|";
                    row_items_number+=1;
                }
                else if action == "STARTMARK"{
                    output+="<mark>"
                }
                else if action == "ENDMARK"{
                    output+="</mark>"
                }
                else if action == "STARTSUPERSCRIPT"{
                    output+="<sup>";
                }
                else if action == "ENDSUPERSCRIPT"{
                    output+="</sup>";
                }
                else if action == "STARTSUBSCRIPT"{
                    output+="<sub>"
                }
                else if action == "ENDSUBSCRIPT"{
                    output+="</sub>";
                }

                else {
                    println!("failed action: {}", action);
                }

            }
        }
        else if chars[pos] == '_'{
            pos+=1;
            output+="\\_";
        }
        else if chars[pos] == '*'{
            pos+=1;
            output+="\\*";
        }
        else if chars[pos] == '#'{
            pos+=1;
            output+="\\#";
        }
        else if chars[pos] == '<'{
            output+="\\<";
            pos+=1;
        }
        else if chars[pos] == '>'{ 
            output+="\\>";
            pos+=1;
        }
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
                else if action == "SECTION"{
                    output+="<h2>";
                }
                else if action == "STARTMARK"{
                    output+="<mark>";
                }
                else if action == "ENDSECTION"{
                    output+="</h2>";
                }
                else if action == "STARTIMAGE"{
                    output+="<img src=\"";
                }
                else if action == "ENDIMAGE"{
                    output+="\">"
                }
                else if action == "STARTRIGHT"{
                    output+="<div style=\"text-align: right\">";
                }
                else if action == "STARTCENTER"{
                    output+="<div style=\"text-align: center\">";
                }
                else if action == "ENDRIGHT"{ 
                    output+="</div>"
                }
                else if action == "ENDCENTER"{ 
                    output+="</div>"
                }
                else if action == "ENDMARK"{
                    output+="</mark>"
                }
                else if action == "STARTTABLE"{
                    output+="<table>"
                }
                else if action == "ENDTABLE"{
                    output+="</table>"
                }
                else if action == "STARTTABLEITEM"{
                    output+="<th>"
                }
                else if action == "ENDTABLEITEM"{
                    output+="</th>"
                }
                else if action == "STARTTABLEROW"{
                    output+="<tr>"
                }
                else if action == "ENDTABLEROW"{
                    output+="</tr>"
                }
                else if action == "STARTSUPERSCRIPT"{ 
                    output+="<sup>";
                } 
                else if action == "ENDSUPERSCRIPT"{ 
                    output+="</sup>";
                }
                else if action == "STARTSUBSCRIPT"{ 
                    output+="<sub>"
                }
                else if action == "ENDSUBSCRIPT"{ 
                    output+="</sub>";
                }
            }
            
        }
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
                else if action == "STARTLIST"{
                    indents+=1;
                }
                else if action == "ENDLIST"{
                    indents-=1;
                }
                else if action == "STARTLISTITEM"{
                    let mut indents_done = 0;
                    while indents_done < indents{
                        output+="\t";
                        indents_done+=1;
                    }
                    output+="> ";
                }
                else if action == "ENDLISTITEM"{
                    output+="\n";
                }
                else if action == "STARTPARAGRAPH"{
                    output+="\n\t";
                }
                else if action == "ENDPARAGRAPH"{
                    output+="\n";
                }
                else if action == "STARTLINK"{
                    let mut link = String::new();
                    while chars[pos] != '\\'{
                        link +=&chars[pos].to_string();
                        pos+=1;
                    }
                    pos+=1;
                    output+=&("(LINK: \"".to_string() + &(link + "\" Text: "));
                }
                else if action == "EMDLINK"{
                    output+=")";
                }
                else if action == "SECTION"{
                    output+="=========================\n";
                }
                else if action == "ENDSECTION"{
                    output+="\n=========================\n";
                }
                else if action == "STARTRIGHT"{output+="<div style=\"text-align: right\">"}
                else if action == "ENDRIGHT"{output+="</div>"}
                else if action == "STARTCENTER"{output+="<div style=\"text-align: center\">"}
                else if action == "ENDCENTER"{output+="</div>"}
                else if action == "STARTMARK"{output+="<mark>"}
                else if action == "ENDMARK"{output+="</mark>"}
                

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
