use clap::Parser;
use std::fs::create_dir;

mod create_files;

use crate::create_files::create_component_stories_file::create_component_stories_file;
use crate::create_files::create_component_tsx_file::create_component_tsx_file;
use crate::create_files::create_css_module_file::create_css_module_file;
use crate::create_files::create_test_component_file::create_test_component_file;

/// Simple program to create React components
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the component
    #[clap(short, long)]
    component_name: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let component_name = args.component_name;

    create_dir(&component_name)?;

    create_component_stories_file(&component_name)?;
    create_component_tsx_file(&component_name)?;
    create_css_module_file(&component_name)?;
    create_test_component_file(&component_name)?;

    Ok(())
}
