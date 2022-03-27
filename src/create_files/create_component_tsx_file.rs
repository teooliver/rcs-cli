use std::fs::File;
use std::io::Write;

pub fn create_component_tsx_file(component_name: String) -> std::io::Result<()> {
    let mut file = File::create(format!("{component_name}/{}.tsx", component_name))?;

    let component_tsx_template = format!(
        "import styles from './{component_name}.modules.css';
export const {component_name} = () => {{
    return <div className={{styles.example}}>helllo</div>;
}};"
    );

    write!(file, "{}", component_tsx_template)?;

    Ok(())
}
