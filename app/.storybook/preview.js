import {RecoilRoot} from 'recoil';
import { RouterContext } from "next/dist/shared/lib/router-context";

export const parameters = {
  actions: { argTypesRegex: '^on[A-Z].*' },
  controls: {
    matchers: {
      color: /(background|color)$/i,
      date: /Date$/,
    },
  },
  nextRouter: {
    Provider: RouterContext.Provider,
  },
};

const withRecoil = (Story) => (
  <RecoilRoot>
    <Story />
  </RecoilRoot>
)



export const decorators = [withRecoil];
