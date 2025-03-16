//! [Clap](https://docs.rs/clap/latest/clap/) Version 5 will have a
//! [better help format api](https://github.com/clap-rs/clap/issues/2914),
//! since then I want to work with [handlebars](https://docs.rs/handlebars/latest/handlebars/)
//! to auto generated the help output of nebula cli and nebula registry with markdown.

use std::{
    io::{BufWriter, Read, Seek, Write},
    path::PathBuf,
};

use clap::{Arg, Command};
use color_eyre::eyre::Report;

mod cli;

#[derive(Debug, Default)]
pub struct HelpElement {
    name: String,

    short: String,

    long: String,
}

#[derive(Debug, Default)]
pub struct HelpCmd {
    myself: HelpElement,

    args: Vec<HelpArg>,

    sub_cmds: Vec<HelpCmd>,
}

#[derive(Debug, Default)]
pub struct HelpArg {
    myself: HelpElement,

    default_value: Option<String>,
}

pub fn collect_clap<C: clap::CommandFactory>() -> HelpCmd {
    let command = C::command();

    let mut help_cmd = HelpCmd::default();
    collect_command(&mut help_cmd, &command);

    help_cmd
}

pub fn collect_command(help_el: &mut HelpCmd, clap_cmd: &Command) {
    help_el.myself.name = clap_cmd.get_name().to_owned();
    help_el.myself.short = clap_cmd.get_about().unwrap_or_default().to_string();
    help_el.myself.long = clap_cmd.get_long_about().unwrap_or_default().to_string();

    for clap_arg in clap_cmd.get_arguments() {
        let mut help_arg = HelpArg::default();
        collect_arg(&mut help_arg, clap_arg);
        help_el.args.push(help_arg);
    }

    // iterate over all sub commands
    for sub_cmd in clap_cmd.get_subcommands() {
        let mut help_sub = HelpCmd::default();
        collect_command(&mut help_sub, sub_cmd);
        help_el.sub_cmds.push(help_sub);
    }
}

pub fn collect_arg(help_arg: &mut HelpArg, clap_arg: &Arg) {
    help_arg.myself.name = if let Some(name) = clap_arg.get_long() {
        if let Some(short) = clap_arg.get_short() {
            format!("(-{} | --{})", short, name).to_string()
        } else {
            format!("(--{})", name).to_string()
        }
    } else if let Some(name) = clap_arg.get_short() {
        format!("(-{})", name).to_string()
    } else {
        format!("unnamed argument at {}.", clap_arg.get_index().unwrap_or(42)).to_string()
    };

    help_arg.myself.short = clap_arg.get_help().unwrap_or_default().to_string();
    help_arg.myself.long = clap_arg.get_long_help().unwrap_or_default().to_string();

    if let Some(def_val) = clap_arg.get_default_values().first() {
        if let Some(dev_val) = def_val.to_str() {
            help_arg.default_value = Some(dev_val.to_string());
        }
    }
}

pub trait ClapHelpFormattedWrite {
    fn write_help(&mut self, root_cmd: &HelpCmd) -> Result<(), Box<dyn std::error::Error>>;

    fn write_command(
        writer: &mut impl Write,
        cmd: &HelpCmd,
    ) -> Result<(), Box<dyn std::error::Error>>;

    fn write_element(
        writer: &mut impl Write,
        el: &HelpElement,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Default)]
pub struct ClapHelpMd {
    template: Option<String>,

    out_file: Option<PathBuf>,

    writer: Option<Box<dyn Write>>,
}

pub struct ClapHelpMdBuilder {
    inner: ClapHelpMd,

    template_path: Option<PathBuf>,
}

impl ClapHelpMdBuilder {
    pub fn with_output(mut self, path: PathBuf) -> Self {
        self.inner.out_file = Some(path);
        self
    }

    pub fn with_template(mut self, template_path: PathBuf) -> Self {
        self.template_path = Some(template_path);
        self
    }

    pub fn build(mut self) -> Result<ClapHelpMd, std::io::Error> {
        // check template file if used:
        if let Some(template_path) = self.template_path {
            let mut file = std::fs::File::open(template_path)?;
            let bytes = file.seek(std::io::SeekFrom::End(0))?;
            file.seek(std::io::SeekFrom::Start(0))?;

            let mut buf = String::with_capacity(bytes as usize);
            file.read_to_string(&mut buf)?;
            self.inner.template = Some(buf);
        }

        // create output writer
        if let Some(out_file) = &self.inner.out_file {
            let file = std::fs::File::create(&out_file)?;
            self.inner.writer = Some(Box::new(BufWriter::new(file)));
        } else {
            self.inner.writer = Some(Box::new(BufWriter::new(std::io::stdout())));
        }

        Ok(self.inner)
    }
}

impl ClapHelpMd {
    pub fn new() -> ClapHelpMdBuilder {
        ClapHelpMdBuilder { inner: Self::default(), template_path: None }
    }
}

impl ClapHelpFormattedWrite for ClapHelpMd {
    fn write_help(&mut self, root_cmd: &HelpCmd) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(_template) = &self.template {
            todo!("Implement handlebar template")
        } else {
            let writer = self.writer.as_mut().unwrap();

            write!(writer, "# {}\n\n", root_cmd.myself.name)?;
            Self::write_element(writer, &root_cmd.myself)?;

            write!(writer, "\n\n## Arguments\n\n")?;
            for arg in &root_cmd.args {
                write!(writer, "- ")?;
                Self::write_element(writer, &arg.myself)?;
                write!(writer, "\n")?;
            }

            write!(writer, "\n")?;
            write!(writer, "## Subcommands\n\n")?;
            for sub_cmd in &root_cmd.sub_cmds {
                write!(writer, "- {}: {}\n", sub_cmd.myself.name, sub_cmd.myself.short)?;
            }
            write! {writer, "\n"}?;
            for sub_cmd in &root_cmd.sub_cmds {
                write! {writer, "### {}\n\n", sub_cmd.myself.name}?;
                Self::write_command(writer, sub_cmd)?;
                write!(writer, "\n")?;
            }
            Ok(())
        }
    }

    fn write_command(
        writer: &mut impl Write,
        cmd: &HelpCmd,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::write_element(writer, &cmd.myself)?;

        if !cmd.args.is_empty() {
            write!(writer, "\n\n#### Arguments of {}\n\n", cmd.myself.name)?;
            for arg in &cmd.args {
                write!(writer, "- ")?;
                Self::write_element(writer, &arg.myself)?;
                write!(writer, "\n")?;
            }
        } else {
            write!(writer, "\n")?;
        }

        Ok(())
    }

    fn write_element(
        writer: &mut impl Write,
        el: &HelpElement,
    ) -> Result<(), Box<dyn std::error::Error>> {
        write!(writer, "{}", el.name)?;
        if !el.short.is_empty() {
            write!(writer, ", {}", el.short)?;
        }
        if !el.long.is_empty() {
            write!(writer, "\n\n{}", el.long)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Report> {
    println!("Will create markdown documentation for CLI usage");

    let root = collect_clap::<cli::Cli>();
    let mut formatted_writer = ClapHelpMd::new()
        .with_output("cli.md".into())
        .build()?;
    formatted_writer.write_help(&root).unwrap();

    Ok(())
}

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    " (",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

pub fn version() -> String {
    let author = clap::crate_authors!();

    let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();

    format!(
        "\
{current_exe_path} - {VERSION_MESSAGE}

Authors: {author}"
    )
}
