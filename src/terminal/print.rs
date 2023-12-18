use anyhow::anyhow;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn colored_msg(msg: &str, color: Color) -> anyhow::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "{}", msg).map_err(|e| anyhow!(e))
}
