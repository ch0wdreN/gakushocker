import { ComponentStory, ComponentMeta } from '@storybook/react';
import React from 'react';

import { Product } from '../../generated/graphql';
import { MenuItem } from '@/components/MenuItem';

export default {
  title: 'MenuItem',
  components: MenuItem,
} as ComponentMeta<typeof MenuItem>;

const Template: ComponentStory<typeof MenuItem> = (props: Product) => (
  <MenuItem {...props} />
);

export const AvailableItem = Template.bind({});
AvailableItem.args = {
  id: 1,
  name: 'カツ丼',
  price: 360,
  stock: 100,
};

export const UnAvailableItem = Template.bind({});
UnAvailableItem.args = {
  id: 1,
  name: 'カツ丼',
  price: 360,
  stock: 0,
};
