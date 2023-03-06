export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  NaiveDateTime: any;
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

export type Find_Product_By_IdQueryVariables = Exact<{
  path: Scalars['Int'];
}>;


export type Find_Product_By_IdQuery = { __typename?: 'Query', findProductById: { __typename?: 'Product', id: number, name: string, price: number, stock: number } };

export type Get_Menu_ListQueryVariables = Exact<{ [key: string]: never; }>;


export type Get_Menu_ListQuery = { __typename?: 'Query', listProduct: Array<{ __typename?: 'Product', id: number, name: string, price: number, stock: number }> };
