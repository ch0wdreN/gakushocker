import { Component } from 'solid-js';
import { useParams } from '@solidjs/router';
import { createGraphQLClient, gql } from '@solid-primitives/graphql';
import { Find_Product_By_IdQuery } from '../../generated/graphql';

const API_URL = 'http://localhost:8080';
const getMenuDocument = gql`
  query get_menu_by_id($path: Int!) {
    findProductById(id: $path) {
      id
      name
      price
      stock
    }
  }
`;
const client = createGraphQLClient(API_URL);

const Item: Component = () => {
  const param = useParams();
  const [data] = client<Find_Product_By_IdQuery>(getMenuDocument, {
    path: Number(param.id),
  });

  console.log(JSON.stringify(data.state));
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
              {data()?.findProductById.name}
            </h2>
            <p class="mt-1">￥{data()?.findProductById.price}</p>
            <p class="mt-1">残り{data()?.findProductById.stock}</p>
          </div>
        </div>
      </div>
    </>
  );
};

export default Item;
