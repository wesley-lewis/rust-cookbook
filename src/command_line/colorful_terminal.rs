use ansi_term::{Colour, Style};

pub fn run_beautiful_terminal() {
    println!("Different Colors");
    println!("Red: {}\nBlue: {}\nGreen: {}", Colour::Red.paint("This is red color"),
    Colour::Blue.paint("blue"),
    Colour::Green.paint("green"));

    println!("Some bold text: {}\n Another coloured bold text: {}", 
             Style::new().bold().paint("This is Bold"),
             Colour::Red.bold().paint("This is bold red")
             );
}
