use url::percent_encoding::percent_decode;
use failure::Error;
use structopt::StructOpt;
use std::io::{self, Read};
use atty::Stream;
use log::*;
use pretty_env_logger;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "URL")]
    input: Option<String>,
}

fn read_from_stdin() -> Result<String, Error> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

fn is_stdin(input: Option<&String>) -> bool {
    let is_request = match input {
        Some(i) if i == "-" => true,
        _ => false,
    };

    let is_pipe = ! atty::is(Stream::Stdin);

    is_request || is_pipe
}

fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    debug!("opt: {:?}", opt);

    if opt.input.is_none() && ! is_stdin(opt.input.as_ref()) {
        Opt::clap().print_help()?;
        std::process::exit(1);
    }

    let input = match opt.input {
        Some(i) => i,
        None => read_from_stdin()?
    };

    if input.is_empty() {
        Opt::clap().get_matches().usage();
    }

    Ok(println!("{}", decode(&input)?))
}

fn decode(input: &str) -> Result<String, Error> {
    let decoded = percent_decode(input.as_bytes()).decode_utf8()?;
    Ok(decoded.to_string())
}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn decode_space_ok() {
        let expected = "foo bar";
        let input = "foo%20bar";
        let actual = decode(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_ascii_ok() {
        let expected = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ `"##;
        let input = r##"%20%21%22%23%24%25%26%27%28%29%2A%2B%2C%2D%2E%2F0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ `"##;
        let actual = decode(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn decode_invalid_utf8_ng() {
        let input = "%93%FA%96%7B%8c%EA%0D%0A";
        decode(input).unwrap();
    }
}

