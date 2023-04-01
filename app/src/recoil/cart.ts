import { atom, atomFamily, selector } from 'recoil';
import {OrderItemInput} from "../../generated/graphql";

export const cartIDState = atom<number[]>({
  key: 'cartIDState',
  default: [],
});

export interface OrderItem extends OrderItemInput{
  name: string,
  price: number
}

export const cartItemState = atomFamily<OrderItem, number>({
  key: "cartItemState",
  default: undefined
})

export const cartState = selector<OrderItem[]>(
  {
    key: 'cartState',
    get: ({get}) => {
      const cartIDs = get(cartIDState);
      return cartIDs.map((id) => {
        return get(cartItemState(id));
      }).filter((i) => i.quantity> 0);
    }
  }
)


export const orderInputState = selector<OrderItemInput[]>({
  key: 'orderInputState',
  get: ({get}) => {
    const cartIDs = get(cartIDState);
    return cartIDs.map((id) => {
      let item =  get(cartItemState(id));
      return {productId: item.productId, quantity: item.quantity}
    }).filter((item) => item.quantity > 0);
}}
)

export const totalState = atom({
  key: 'totalState',
  default: 0
})