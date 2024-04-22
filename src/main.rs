use anyhow::Result;
use clap::{ArgAction, Parser};
use file_type::FileType;
use quick_xml::Reader;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::io::BufReader;

mod file_type;
mod xmltojson;

#[derive(Parser, Clone)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// path of the XML file to read from; we use '-' to denote STDIN
    #[clap(short('X'), long("xml-file"), default_value_t = FileType::Stream)]
    xml_file: FileType,

    /// path of the JSON file to write to; we use '-' to denote STDIN
    #[clap(short('J'), long("json-file"), default_value_t = FileType::Stream)]
    json_file: FileType,

    /// added with-text-attr option to allow _text from the output
    #[clap(short('T'), long("with-text-attr"), action = ArgAction::Set, default_value = "true")]
    with_text_attr: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    let input_stream = cli.xml_file.as_reader()?;
    let output_stream = cli.json_file.as_writer()?;
    let with_text_attr = cli.with_text_attr;

    let mut reader = Reader::from_reader(BufReader::new(input_stream));
    //TODO: option?
    reader.trim_text(true);

    let val = xmltojson::read(&mut reader, 0, with_text_attr);

    serde_json::ser::to_writer(output_stream, &val)?;
    Ok(())
}
