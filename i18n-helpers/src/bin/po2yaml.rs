use anyhow::{anyhow, Context};
use polib::po_file;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize, PartialEq, Debug)]
struct Message<'a> {
    msgid: &'a str,
    msgstr: &'a str,
}

fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or(anyhow!("usage: po2yaml <po-file>"))?;
    let catalog = po_file::parse(Path::new(&path))
        .map_err(|err| anyhow!("{err}"))
        .with_context(|| format!("Could not parse {path} as PO file"))?;

    let mut messages = Vec::new();
    for msg in catalog.messages.iter() {
        messages.push(Message {
            msgid: msg.get_msgid().unwrap(),
            msgstr: msg.get_msgstr().unwrap(),
        });
    }

    println!("{}", serde_yaml::to_string(&messages)?);
    Ok(())
}
