import { Component, createSignal, For } from 'solid-js';
import { createGraphQLClient, gql } from '@solid-primitives/graphql';
import { Get_Menu_ListQuery, Product } from '../../generated/graphql';
import Card from '~/component/Card';
import { createStore } from 'solid-js/store';

const API_URL = 'http://localhost:8080';
const listMenuDocument = gql`
  query get_all_menu {
    listProduct {
      id
      name
      price
      stock
    }
  }
`;
const client = createGraphQLClient(API_URL);

interface Order {
  menu: string[];
  price: number;
}

const Menu: Component = () => {
  const [isOrder, setIsOrder] = createSignal(false);
  const [orders, setOrders] = createStore<Order[]>([]);
  const [data] = client<Get_Menu_ListQuery>(listMenuDocument);
  return (
    <>
      <div>
        <span>{data.loading && 'Loading...'}</span>
        <section class="text-gray-600 body-font">
          <div class="container px-5 py-24 mx-auto">
            <div class="flex flex-wrap -m-4">
              <For each={data()?.listProduct}>
                {(menu: Product) => {
                  return <Card {...menu} />;
                }}
              </For>
            </div>
          </div>
        </section>
        {isOrder() ? <p>注文を確定する</p> : <></>}
      </div>
    </>
  );
};
export default Menu;
