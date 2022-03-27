use std::fs::File;
use std::io::Write;

pub fn create_css_module_file(component_name: String) -> std::io::Result<()> {
    let mut file = File::create(format!("{component_name}/{}.module.css", component_name))?;

    let css_module_template = format!(
        ".example {{
  background-color: red;
}}
"
    );

    write!(file, "{}", css_module_template)?;

    Ok(())
}
