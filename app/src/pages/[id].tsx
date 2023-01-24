import { Component } from 'solid-js';
import { useParams } from '@solidjs/router';
import { createGraphQLClient, gql } from '@solid-primitives/graphql';
import { Get_Menu_By_IdQuery } from '../../generated/graphql';

const API_URL = 'http://localhost:8080';
const getMenuDocument = gql`
  query get_menu_by_id($path: Int!) {
    getMenu(id: $path) {
      id
      menu
      price
      stock
    }
  }
`;

const Item: Component = () => {
  const param = useParams();
  console.log(param.id);
  const client = createGraphQLClient(API_URL);
  const [data] = client<Get_Menu_By_IdQuery>(getMenuDocument, {
    path: param.id,
  });

  console.log(JSON.stringify(data()?.getMenu));
  return (
    <>
      <div class="container px-5 py-24 mx-auto">
        <span>{data.loading && 'Loading...'}</span>
        <div class="lg:w-1/4 md:w-1/2 p-4 w-full">
          <div class="block relative h-48 rounded overflow-hidden">
            <img
              alt="ecommerce"
              class="object-cover object-center w-full h-full block"
              src="https://dummyimage.com/420x260"
            />
          </div>
          <div class="mt-4">
            <h2 class="text-gray-900 title-font text-lg font-medium">
              {data()?.getMenu.menu}
            </h2>
            <p class="mt-1">￥{data()?.getMenu.price}</p>
            <p class="mt-1">残り{data()?.getMenu.stock}</p>
          </div>
        </div>
      </div>
    </>
  );
};

export default Item;
