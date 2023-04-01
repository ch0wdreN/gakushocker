import { ComponentStory, ComponentMeta } from '@storybook/react';
import React from 'react';
import { RecoilRoot, MutableSnapshot } from 'recoil';

import { Header } from '@/components/Header';
import { cartIDState, cartItemState } from '@/recoil/cart';
import { isSigninState } from '@/recoil/signin';
import { userState } from '@/recoil/user';

export default {
  title: 'Header',
  components: Header,
} as ComponentMeta<typeof Header>;

const initializeSignInState = ({ set }: MutableSnapshot) => {
  set(isSigninState, true);
  set(userState, {
    id: 1,
    displayName: 'demo',
    email: 'demo@email.com',
    password: 'password',
    point: 9999,
  });
  set(cartIDState, [1, 2]);
  set(cartItemState(1), {
    productId: 1,
    quantity: 1,
    name: 'カツ丼',
    price: 360,
  });
  set(cartItemState(2), {
    productId: 2,
    quantity: 1,
    name: '肉うどん',
    price: 300,
  });
};

export const SignIn: ComponentStory<typeof Header> = () => (
  <RecoilRoot initializeState={initializeSignInState}>
    <Header />
  </RecoilRoot>
);

export const NotSignIn: ComponentStory<typeof Header> = () => <Header />;
