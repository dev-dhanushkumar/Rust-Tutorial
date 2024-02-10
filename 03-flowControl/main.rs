mod ifelse;
mod infinate_loop;
mod nesting_label;
mod whileloop;
mod forloop;
mod match_pattern;

use forloop::for_loop;
use whileloop::while_loop;
use infinate_loop::infiloop;
use ifelse::if_else;
use nesting_label::label_loop;
use match_pattern::match_values;

fn main() {
    if_else();
    println!("\n\n");
    infiloop();
    println!("\n\n");
    label_loop();
    println!("\n\n");
    while_loop();
    println!("\n\n");
    for_loop();
    println!("\n \n");
    match_values();
}