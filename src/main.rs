mod logic;
use chrono;
use chrono::Datelike;
mod settings;
use std::env;
mod format;
use std::fs;


//general information about the document
pub struct docinfo{
    title: String,
    font: String,
}


// define the standard vars.
pub fn std_vars() -> Vec<Vec<String>>{
    let mut out:Vec<Vec<String>> = vec![];
    out.push(["version".to_string(), settings::version()].to_vec());
    out.push(["year".to_string(), chrono::Utc::now().year().to_string()].to_vec());
    out.push(["month".to_string(), chrono::Utc::now().month().to_string()].to_vec());
    out.push(["day".to_string(), chrono::Utc::now().day().to_string()].to_vec());
    return out;
}

fn main() {
    let frog = include_str!("sexy frog.txt");
    let help = include_str!("help info.txt");
    let mut infile = "input.txt".to_string();    
    let mut outfile = "out.md".to_string();
    let mut format = "text".to_string();
    let args: Vec<_> = env::args().collect();
    
    let mut x = 1;
    while x < args.len(){
        if args[x] == "-h" ||args[x] == "--help"{
            println!("{}", help);
            std::process::exit(0);
        }
        else if args[x] == "-o" || args[x] == "--output"{
            outfile = args[x+1].clone();
            x+=2;
        }
        else if args[x] == "frog" || args[x] == "--frog"{
            println!("{frog}");
            println!("It's not what it looks like... I swear...");
            std::process::exit(69);
        }
        else if args[x] == "-f" ||  args[x] == "--format"{
            format = args[x+1].clone();
            x+=2;
        }
        else if args[x] == "-i" || args[x] == "--input"{
            infile = args[x+1].clone();
            x+=2;
        }
        else{
            println!("ERROR: Unknown argument \"{}\"", args[x]);
            std::process::exit(1);
        }
    }



    if infile == ""{
        println!("Error: no input file specified.");
        std::process::exit(1);
    }

    let file_content = logic::read_file(&infile);
    let logical_parser_output = logic::logical_parser(&file_content, std_vars());
    //println!("lp: {logical_parser_output}");
    //println!("\n\n\n\n\n\n\n\n\n");
    let format_parser_output = format::format_parser(&logical_parser_output.0);
    //println!("IR: {format_parser_output}");

    if format == "markdown"{
        format::markdown_parser(&format_parser_output, &outfile, logical_parser_output.2);
    }
    else if format == "IR"{
        fs::write(outfile, format_parser_output + &("\n::::::::::\nTITLE:".to_owned() + &logical_parser_output.2.title));
    }
    else if format == "HTML"{
        format::HTML_parser(&format_parser_output, &outfile, logical_parser_output.2);
    }
    else if format == "text"{
        format::text_parser(&format_parser_output, &outfile, logical_parser_output.2);
    }
    else{
        println!("error: unknown format");
        std::process::exit(1);
    }

}
