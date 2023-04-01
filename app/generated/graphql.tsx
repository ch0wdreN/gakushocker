import gql from 'graphql-tag';
import * as Urql from 'urql';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  /**
   * ISO 8601 combined date and time without timezone.
   *
   * # Examples
   *
   * * `2015-07-01T08:59:60.123`,
   */
  NaiveDateTime: any;
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: any;
};

export type Mutation = {
  __typename?: 'Mutation';
  createOrder: Order;
  createProduct: Product;
  createUser: User;
  deleteOrder: Order;
  deleteProduct: Product;
  deleteUser: User;
};


export type MutationCreateOrderArgs = {
  input: OrderInput;
};


export type MutationCreateProductArgs = {
  input: ProductInput;
};


export type MutationCreateUserArgs = {
  input: UserInput;
};


export type MutationDeleteOrderArgs = {
  id: Scalars['UUID'];
};


export type MutationDeleteProductArgs = {
  id: Scalars['Int'];
};


export type MutationDeleteUserArgs = {
  id: Scalars['Int'];
};

export type Order = {
  __typename?: 'Order';
  createdAt: Scalars['NaiveDateTime'];
  id: Scalars['UUID'];
  items: Array<OrderItem>;
  status: Scalars['String'];
  total: Scalars['Int'];
  userId: Scalars['Int'];
};

export type OrderInput = {
  id: Scalars['UUID'];
  items: Array<OrderItemInput>;
  status: Scalars['String'];
  total: Scalars['Int'];
  userId: Scalars['Int'];
};

export type OrderItem = {
  __typename?: 'OrderItem';
  name: Scalars['String'];
  price: Scalars['Int'];
  quantity: Scalars['Int'];
};

export type OrderItemInput = {
  productId: Scalars['Int'];
  quantity: Scalars['Int'];
};

export type Product = {
  __typename?: 'Product';
  id: Scalars['Int'];
  name: Scalars['String'];
  price: Scalars['Int'];
  stock: Scalars['Int'];
};

export type ProductInput = {
  name: Scalars['String'];
  price: Scalars['Int'];
  stock: Scalars['Int'];
};

export type Query = {
  __typename?: 'Query';
  findAllOrder: Array<Order>;
  findOrderById: Order;
  findProductById: Product;
  findUserByEmail: User;
  listOrder: Array<Order>;
  listProduct: Array<Product>;
  listUser: Array<User>;
};


export type QueryFindOrderByIdArgs = {
  id: Scalars['UUID'];
};


export type QueryFindProductByIdArgs = {
  id: Scalars['Int'];
};


export type QueryFindUserByEmailArgs = {
  email: Scalars['String'];
};

export type User = {
  __typename?: 'User';
  displayName: Scalars['String'];
  email: Scalars['String'];
  id: Scalars['Int'];
  password: Scalars['String'];
  point: Scalars['Int'];
};

export type UserInput = {
  displayName: Scalars['String'];
  email: Scalars['String'];
  password: Scalars['String'];
  point: Scalars['Int'];
};

export type CreateOrderMutationVariables = Exact<{
  input: OrderInput;
}>;


export type CreateOrderMutation = { __typename?: 'Mutation', createOrder: { __typename?: 'Order', id: any, userId: number, total: number, createdAt: any, status: string, items: Array<{ __typename?: 'OrderItem', name: string, price: number, quantity: number }> } };

export type ListProductQueryVariables = Exact<{ [key: string]: never; }>;


export type ListProductQuery = { __typename?: 'Query', listProduct: Array<{ __typename?: 'Product', id: number, name: string, price: number, stock: number }> };


export const CreateOrderDocument = gql`
    mutation createOrder($input: OrderInput!) {
  createOrder(input: $input) {
    id
    userId
    items {
      name
      price
      quantity
    }
    total
    createdAt
    status
  }
}
    `;

export function useCreateOrderMutation() {
  return Urql.useMutation<CreateOrderMutation, CreateOrderMutationVariables>(CreateOrderDocument);
};
export const ListProductDocument = gql`
    query listProduct {
  listProduct {
    id
    name
    price
    stock
  }
}
    `;

export function useListProductQuery(options?: Omit<Urql.UseQueryArgs<ListProductQueryVariables>, 'query'>) {
  return Urql.useQuery<ListProductQuery, ListProductQueryVariables>({ query: ListProductDocument, ...options });
};