use std::env;
use std::fs::File;
use std::io::Write;

mod create_files;

// use create_files::create_component_tsx_file;
use crate::create_files::create_component_tsx_file::create_component_tsx_file;

fn main() -> std::io::Result<()> {
    // Bring that from CLI
    let component_name = "COMPONENT_NAME".to_string();

    // let mut file = File::create(format!("{}.tsx", component_name))?;

    // let typescript_tsx_template = format!(
    //     "import styles from '{}.modules.css';

    // export const NAME_OF_THE_COMPONENTE = () => {{
    //     return <div className={{styles.example}}>helllo</div>;
    // }};",
    //     component_name
    // );
    // write!(file, "{}", typescript_tsx_template);

    create_component_tsx_file(component_name);

    let css_module_template = format!(
        "
    .example {{
        background-color: red;
    }}"
    );

    Ok(())
}
