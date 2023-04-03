mod logic;
use chrono;
use chrono::Datelike;
use std::env;
mod format;
use std::fs;

const VERSION: &str = env!("CARGO_PKG_VERSION");

//general information about the document
pub struct DocInfo{
    title: String,
    font: String,
    bg_color: String,
    bg_image: String,
    page_color: String,
    page_padding: i8, //in integer percentage
    text_padding: i8, //in integer percentage
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
    return out;
}

fn main() {
    let frog = include_str!("sexy frog.txt");
    let help = include_str!("help info.txt");
    let mut infile = "".to_string();    
    let mut outfile = "out.md".to_string();
    let mut format = "markdown".to_string();
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
        fs::write(outfile, format_parser_output + &("\n::::::::::\nTITLE:".to_owned() + &logical_parser_output.2.title)).expect("File system error.");
    }
    else if format == "HTML" || format == "html"{
        format::html_parser(&format_parser_output, &outfile, logical_parser_output.2);
    }
    else if format == "text"{
        format::text_parser(&format_parser_output, &outfile, logical_parser_output.2);
    }
    else if format == "logic"{
        fs::write(outfile, logical_parser_output.0).expect("File System error");
    }
    else{
        println!("error: unknown format");
        std::process::exit(1);
    }

}
