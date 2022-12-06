import type { MenuRecord } from '../../generated/graphql';
import { Component } from 'solid-js';
import '../styles/Card.scss';

interface Prop {
  menu: string,
  price: number,
  stock: number
}

const Card: Component<Prop> = (prop) => {
  return (
    <div class="wrap">
      <p>{prop.menu}</p>
      <p>
        ￥{prop.price} 残り{prop.stock}枚
      </p>
    </div>
  );
};

export default Card;
