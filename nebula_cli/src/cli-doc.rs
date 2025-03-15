//! [Clap](https://docs.rs/clap/latest/clap/) Version 5 will have a 
//! [better help format api](https://github.com/clap-rs/clap/issues/2914), 
//! since then I want to work with [handlebars](https://docs.rs/handlebars/latest/handlebars/) 
//! to auto generated the help output of nebula cli with markdown.

use color_eyre::eyre::Report;

fn main() -> Result<(), Report> {
    println!("Will create markdown documentation for CLI usage");
    Ok(())
}