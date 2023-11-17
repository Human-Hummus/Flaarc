extern crate termion;

mod logic;
mod format;
mod standard_variables;
#[macro_use]
pub mod output;


pub struct Document{
    files: Vec<DocInfo>,
    format: String,
    vars: Vec<Vec<String>>
}


//general information about the document
pub struct DocInfo{
    title: String,
    font: String,
    bg_color: String,
    bg_image: String,
    page_color: String,
    page_padding: i8, //in integer percentage
    text_padding: i8, //in integer percentage

    filename: String,
    content: String,
    outfilename: String
}

impl DocInfo{
    fn clone(&self) -> DocInfo{
        return DocInfo {
            title: self.title.clone(),
            font: self.font.clone(),
            bg_color: self.bg_color.clone(),
            bg_image: self.bg_image.clone(),
            page_color: self.page_color.clone(),
            page_padding: self.page_padding,
            text_padding: self.text_padding,
            filename: self.filename.clone(),
            content: self.content.clone(),
            outfilename: self.outfilename.clone()
        }
    }
}

fn switch_filename(current_filename: &String, new_ex: &String) -> String{
    let text:Vec<char> = current_filename.chars().collect();
    let mut number_of_dots = 0;
    let mut x = 0;
    while x < text.len(){
        if text[x] == '.'{number_of_dots+=1}
        x+=1;
    }
    
    if number_of_dots == 0 {return current_filename.to_string() + new_ex;}
    x-=1;

    while text[x] != '.'{
        x-=1;
    }

    let mut out = String::new();
    let mut y = 0;
    while y<x{
        out.push(text[y]);
        y+=1;
    }

    return out + new_ex;

}

fn default_docinfo(filename: String, format: &String) -> DocInfo{
    let mut tmp = DocInfo{
        title: "Title".to_string(),
        font: "times".to_string(),
        bg_color: "white".to_string(),
        bg_image: "".to_string(),
        page_color: "white".to_string(),
        page_padding: 5,
        text_padding: 0,
        filename: filename.clone(),
        content: "".to_string(),
        outfilename: filename.clone(),
    };
    if format == "markdown"{
        tmp.outfilename = switch_filename(&filename, &".md".to_string());
    }
    else if format == "text"{ 
        tmp.outfilename = switch_filename(&filename, &".txt".to_string());
    }
    else if format == "html"{
        tmp.outfilename = switch_filename(&filename, &".html".to_string());
    }
    return tmp;

}



fn main() {
    let mut infile = "".to_string(); 
    let mut outfile = "output.html".to_string();
    let mut format = "html".to_string();
    let args: Vec<_> = std::env::args().collect();

    let mut x = 1;
    while x < args.len(){match args[x].as_str(){
        "-h" | "--help" => {
            alert!(include_str!("help info.txt"));
            std::process::exit(0);
        }
        "-o" | "--output" => {
            outfile = args[x+1].clone();x+=2}
        "frog" | "--frog" => {
            alert!(include_str!("sexy frog.txt"));
            alert!("It's not what it looks like... I swear...");
            std::process::exit(69)}
        "-f" | "--format" => {
            format = args[x+1].clone();x+=2},
        "-i" | "--input" => {
            infile = args[x+1].clone();x+=2},
        _ => fatal!(format!("ERROR: Unknown argument \"{}\"", args[x]))
        }
    }



    if infile == ""{fatal!("Error: no input file specified.")}
    let mut document = Document {vars: standard_variables::std_vars(), files: vec![default_docinfo(infile.clone(), &format)], format: format};
    document.files[0].outfilename = outfile;

    let file_content = logic::read_file(&infile);
    let df0 = document.files[0].clone();
    let logical_parser_output = logic::logical_parser(&file_content, document, df0, false);
    document = logical_parser_output.1;
    document.files[0] = logical_parser_output.2;

    let mut x = 0;

    while x < document.files.len(){
        debug!(format!("stuff: {}", document.files[x].content));
        let mut vars = document.vars.clone();
        vars.push(vec!["title".to_string(), document.files[x].title.clone()]);
        fmt_file(document.files[x].clone(), &document.format, &document, &vars);
        x+=1;
    }

}


fn fmt_file(file: DocInfo, format: &String, document: &Document, vars: &Vec<Vec<String>>){
    debug!(format!("exporting {} to {} with {}", &file.filename, &file.outfilename, &file.content));
    let format_parser_output = format::format_parser(&file.content, document);
    format::output_file(format, &file.outfilename, &format_parser_output, vars);

}
