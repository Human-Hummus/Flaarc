use std::fs;
use std::env;
use std::convert::TryInto;

fn read_file(filename:&String) -> String{
    let data = fs::read_to_string(filename).unwrap_or_else(|_error|{
        println!("Warning: unable to read file with name \"{}\"", filename);
        return "\nFILE SYSTEM ERROR\n".to_string();
    });
    return data.to_string();
}

fn main(){
    let args: Vec<_> = env::args().collect();
    let file_content = read_file(&args[1]);
    
    if file_content == "\nFILE SYSTEM ERROR\n".to_string(){
        println!("\n\nFAILED TO IMPORT TABLE; FS ERROR.\n\n")
    }
    let mut table:Vec<Vec<String>> = vec![];
    let mut numcols = 0;
    let mut longest_cells:Vec<usize> = vec![];

    let mut x = 0;
    let chars:Vec<char> = file_content.chars().collect();
    
    let mut working_row:Vec<String> = vec![];
    let mut working_cell:String = String::new();


    while x < chars.len(){
        if chars[x] == ','{
            working_row.push(working_cell.clone());
            if working_row.len() > longest_cells.len(){
                longest_cells.push(working_cell.len().try_into().unwrap());
            }
            else if working_cell.len() > longest_cells[working_row.len()]{
                longest_cells[working_row.len()] = working_cell.len().try_into().unwrap();
            }
            working_cell = String::new();
        }
        else if chars[x] == '\n'{
            if working_cell.len() > 0{
                working_row.push(working_cell.clone());
                let mut wrl:usize = working_row.len().try_into().unwrap();
                if working_row.len() > longest_cells.len(){
                    longest_cells.push(working_cell.len().try_into().unwrap());
                }
                else {
                    if working_cell.len() > longest_cells[wrl-1]{
                        longest_cells[working_row.len()] = working_cell.len().try_into().unwrap();
                    }
                }
                working_cell = String::new();
            }

            table.push(working_row);
            working_row = vec![];
        }
        else{
            working_cell+=&chars[x].to_string();
        }
        x+=1;
    }
    if working_cell.len() > 0{ 
        working_row.push(working_cell.clone());
        if working_row.len() > longest_cells.len(){
            longest_cells.push(working_cell.len().try_into().unwrap());
        }
        else if working_cell.len() > longest_cells[working_row.len()].try_into().unwrap(){
            longest_cells[working_row.len()] = working_cell.len().try_into().unwrap();
        }
        working_cell = String::new();
    }
            
    table.push(working_row);
    working_row = vec![];

    table = ensure_widths_same(&longest_cells, table);
    let mut tmp = 0;
    print!("┌");
    while tmp < longest_cells.iter().sum::<usize>() + longest_cells.len()-1{
        print!{"─"};
        tmp+=1;
    }
    print!("┐\n");

    let mut crow = 0;
    let mut chr = 0;

    println!("{:?}, {}", table, longest_cells.len());


    while crow < table.len(){
        print!("│");
        while chr < longest_cells.len(){
            print!("{}│", table[crow][chr]);
            chr+=1;
        }
        if crow != table.len()-1{
            print!("\n├");
            tmp = 0;
            while tmp < gsm(&longest_cells) + longest_cells.len()-1{ 
                print!{"─"};
                tmp+=1;
            }
            print!("┤\n");
            
        }
        else {
            print!("\n");
        }
        chr=0;
        crow+=1;
        
    }
    let mut tmp = 0;
    print!("└");
    while tmp < longest_cells.iter().sum::<usize>() + longest_cells.len()-1{ 
        print!{"─"};
        tmp+=1;
    }
    print!("┘\n");
}





//ensure all of the widths of the cells and columns are the same.
fn ensure_widths_same(cell_widths: &Vec<usize>, mut table: Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut current_row = 0;
    let mut current_cell = 0;

    while current_row < table.len(){
        //make the row the right length
        while table[current_row].len() < cell_widths.len(){
            table[current_row].push(String::new());
        }

        //make the cells in the row all the same
        while current_cell < cell_widths.len(){
            while table[current_row][current_cell].len() < cell_widths[current_cell]{
                table[current_row][current_cell]+=" ";
            }
            current_cell+=1;
        }
        current_cell = 0;
        current_row+=1;
    }
    return table;
}



fn gsm(x: &Vec<usize>) -> usize{
    let mut total = 0;
    let mut y = 0;
    while y < x.len(){
        total+=x[y];
        y+=1;
    }
    return total

}
