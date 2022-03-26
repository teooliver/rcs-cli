use std::fs::File;
use std::io::Write;

pub fn create_component_tsx_file(component_name: String) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.tsx", component_name))?;

    let typescript_tsx_template = format!(
        "import styles from '{}.modules.css';

    export const NAME_OF_THE_COMPONENTE = () => {{
        return <div className={{styles.example}}>helllo</div>;
    }};",
        component_name
    );

    write!(file, "{}", typescript_tsx_template);

    Ok(())
}
