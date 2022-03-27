use std::fs::File;
use std::io::Write;

pub fn create_component_stories_file(component_name: String) -> std::io::Result<()> {
    let mut file = File::create(format!("{component_name}/{}.stories.tsx", component_name))?;

    let component_tsx_template = format!(
"import {{ ComponentStory, ComponentMeta }} from '@storybook/react';
import {{ {component_name} }} from './{component_name}';

// More on default export: https://storybook.js.org/docs/react/writing-stories/introduction#default-export
export default {{
  title: '{component_name}',
  component: {component_name},
}} as ComponentMeta<typeof {component_name}>;

// More on component templates: https://storybook.js.org/docs/react/writing-stories/introduction#using-args
const Template: ComponentStory<typeof {component_name}> = () => (
  <{component_name} />
);

export const Primary = Template.bind({{}});
// More on args: https://storybook.js.org/docs/react/writing-stories/args
Primary.args = {{
  primary: true,
  label: '{component_name}',
}};
"
    );

    write!(file, "{}", component_tsx_template)?;

    Ok(())
}
