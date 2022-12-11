import { Component, For } from 'solid-js';
import { createGraphQLClient, gql } from '@solid-primitives/graphql';
import { Get_All_MenuQuery, MenuRecord } from '../../generated/graphql';
import Card from '~/component/Card';

const API_URL = 'http://localhost:8080';
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
const client = createGraphQLClient(API_URL);

const Menu: Component = () => {
  const [data] = client<Get_All_MenuQuery>(getAllMenuDocument);
  return (
    <>
      <section class="text-gray-600 body-font">
        <div class="container px-5 py-24 mx-auto">
          <div class="flex flex-wrap -m-4">
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
          </div>
        </div>
      </section>
    </>
  );
};
export default Menu;
