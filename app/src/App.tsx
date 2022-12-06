import type { Component } from 'solid-js';
import { gql, createGraphQLClient } from '@solid-primitives/graphql';
import { Get_All_MenuQuery, MenuRecord } from '../generated/graphql';

import styles from './App.module.css';
import { For } from 'solid-js';
import Card from '~/component/Card';

const App: Component = () => {
  const getAllMenuDocument = gql`
    query get_all_menu {
      getAllMenu {
          id
        menu
        price
        stock
      }
    }
  `;
  const endpoint = 'http://localhost:8000';
  const client = createGraphQLClient(endpoint);
  const [data] = client<Get_All_MenuQuery>(getAllMenuDocument);
  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <For each={data()?.getAllMenu}>
          {(menu: MenuRecord) => {
            return (
              <Card
                menu={menu.menu}
                stock={menu.stock}
                price={menu.price}
              />
            );
          }}
        </For>
      </header>
    </div>
  );
};

export default App;
