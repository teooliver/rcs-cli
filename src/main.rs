use std::env;
use std::fs::create_dir;
use std::io::Write;

mod create_files;

// use create_files::create_component_tsx_file;
use crate::create_files::create_component_tsx_file::create_component_tsx_file;

fn main() -> std::io::Result<()> {
    // Bring that from CLI
    let component_name = "ComponentName".to_string();

    create_dir(&component_name)?;

    create_component_tsx_file(component_name.to_string());

    let css_module_template = format!(
        "
    .example {{
        background-color: red;
    }}"
    );

    Ok(())
}
