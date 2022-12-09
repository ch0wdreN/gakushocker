import { Component } from 'solid-js';

interface Prop {
  menu: string;
  price: number;
  stock: number;
}

const Card: Component<Prop> = (prop) => {
  return (
    <div class="lg:w-1/4 md:w-1/2 p-4 w-full">
      <a class="block relative h-48 rounded overflow-hidden">
        <img
          alt="ecommerce"
          class="object-cover object-center w-full h-full block"
          src="https://dummyimage.com/420x260"
        />
      </a>
      <div class="mt-4">
        <h2 class="text-gray-900 title-font text-lg font-medium">
          {prop.menu}
        </h2>
        <p class="mt-1">￥{prop.price}</p>
        <p class="mt-1">残り{prop.stock}</p>
      </div>
    </div>
  );
};

export default Card;
