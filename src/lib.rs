use anyhow::Result;
use vizia::prelude::{Context, ImageRetentionPolicy};
use vizia::util::CSS;

#[macro_use]
mod macros;

pub fn load_assets(cx: &mut Context) -> Result<()> {
    include!(concat!(env!("OUT_DIR"), "/load_assets_body"))
}
