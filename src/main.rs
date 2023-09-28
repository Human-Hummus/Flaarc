mod logic;
use chrono;
use chrono::Datelike;
use std::env;
mod format;
use std::fs;
extern crate termion;
#[macro_use]
pub mod output;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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


// define the standard vars.
pub fn std_vars() -> Vec<Vec<String>>{
    let mut out:Vec<Vec<String>> = vec![];
    out.push(["version".to_string(), VERSION.to_string()].to_vec());
    out.push(["year".to_string(), chrono::Utc::now().year().to_string()].to_vec());
    out.push(["month".to_string(), chrono::Utc::now().month().to_string()].to_vec());
    out.push(["day".to_string(), chrono::Utc::now().day().to_string()].to_vec());
    out.push(["invbang".to_string(), "¡".to_string()].to_vec());
    out.push(["copy".to_string(), "©".to_string()].to_vec());
    out.push(["reg".to_string(), "®".to_string()].to_vec());
    out.push(["deg".to_string(), "°".to_string()].to_vec());
    out.push(["plusmn".to_string(), "±".to_string()].to_vec());
    out.push(["micro".to_string(), "µ".to_string()].to_vec());
    
    //these are unicode combining chars, you can't see them, but they're there!
    out.push(["accute".to_string(), " ́".to_string()].to_vec());
    out.push(["overline".to_string(), " ̅".to_string()].to_vec());
    out.push(["diaresis".to_string(), " ̈".to_string()].to_vec());
    //done with the unicode now*!
    
    out.push(["theta".to_string(), "ϴ".to_string()].to_vec());
    out.push(["omega".to_string(), "Ω".to_string()].to_vec());
    out.push(["alpha".to_string(), "α".to_string()].to_vec());
    out.push(["beta".to_string(), "β".to_string()].to_vec());
    out.push(["gamma".to_string(), "γ".to_string()].to_vec());
    out.push(["delta".to_string(), "Δ".to_string()].to_vec());
    out.push(["sigma".to_string(), "Σ".to_string()].to_vec());
    out.push(["pi".to_string(), "π".to_string()].to_vec());
    out.push(["cap_omega".to_string(), "Ѡ".to_string()].to_vec());
    out.push(["bullet".to_string(), "•".to_string()].to_vec());
    out.push(["block".to_string(), "█".to_string()].to_vec());
    out.push(["light_shade".to_string(), "░".to_string()].to_vec());
    out.push(["med_shade".to_string(), "▒".to_string()].to_vec());
    out.push(["dark_shade".to_string(), "▓".to_string()].to_vec());

    //emojis!!!!
    out.push(["thunder".to_string(), "⚡".to_string()].to_vec());
    out.push(["fist".to_string(), "✊".to_string()].to_vec());
    out.push(["check".to_string(), "✔".to_string()].to_vec());
    out.push(["x".to_string(), "✘".to_string()].to_vec());
    out.push(["sparkle".to_string(), "✨".to_string()].to_vec());
    out.push(["cross".to_string(), "❌".to_string()].to_vec());
    out.push(["black_heart".to_string(), "❤".to_string()].to_vec());
    out.push(["smile".to_string(), "😀".to_string()].to_vec());
    out.push(["veryfunny".to_string(), "😂".to_string()].to_vec());
    out.push(["smile2".to_string(), "😃".to_string()].to_vec());
    out.push(["embarrassment".to_string(), "😅".to_string()].to_vec());
    out.push(["cool".to_string(), "😎".to_string()].to_vec());
    out.push(["smirk".to_string(), "😏".to_string()].to_vec());
    out.push(["apathy".to_string(), "😐".to_string()].to_vec());
    out.push(["crying".to_string(), "😢".to_string()].to_vec());
    out.push(["ono".to_string(), "😳".to_string()].to_vec());
    out.push(["Smile".to_string(), "🙂".to_string()].to_vec());
    out.push(["nerd".to_string(), "🤓".to_string()].to_vec());
    out.push(["brain".to_string(), "🧠".to_string()].to_vec());
    out.push(["cap".to_string(), "🧢".to_string()].to_vec());
    out.push(["frog".to_string(), include_str!("sexy frog.txt").to_string()].to_vec());
    out.push(["div".to_string(), "÷".to_string()].to_vec());
    return out;
}

fn main() {
    let mut infile = "".to_string(); 
    let mut outfile = "output.html".to_string();
    let mut format = "html".to_string();
    let args: Vec<_> = env::args().collect();

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
    let mut document = Document {vars: std_vars(), files: vec![default_docinfo(infile.clone(), &format)], format: format};
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
