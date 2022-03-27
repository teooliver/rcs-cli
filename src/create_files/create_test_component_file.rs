use std::fs::File;
use std::io::Write;

pub fn create_test_component_file(component_name: String) -> std::io::Result<()> {
    let mut file = File::create(format!("{component_name}/{}.test.tsx", component_name))?;

    let component_test_template = format!(
        "// import {{ render, screen, fireEvent }} from '@testing-library/react';

describe('Test {component_name} Component', () => {{
  test('Can render {component_name}', async () => {{
    // Right your tests here
  }});
}});
    "
    );

    write!(file, "{}", component_test_template)?;

    Ok(())
}
