import { ComponentStory, ComponentMeta } from '@storybook/react';
import COMPONENT_NAME from './COMPONENT_NAME';

// More on default export: https://storybook.js.org/docs/react/writing-stories/introduction#default-export
export default {
  title: 'COMPONENT_NAME',
  component: COMPONENT_NAME,
} as ComponentMeta<typeof COMPONENT_NAME>;

// More on component templates: https://storybook.js.org/docs/react/writing-stories/introduction#using-args
const Template: ComponentStory<typeof COMPONENT_NAME> = () => (
  <COMPONENT_NAME />
);

export const Primary = Template.bind({});
// More on args: https://storybook.js.org/docs/react/writing-stories/args
Primary.args = {
  primary: true,
  label: 'COMPONENT_NAME',
};
