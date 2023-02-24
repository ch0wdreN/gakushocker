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
};

export type Mutation = {
  __typename?: 'Mutation';
  createOrder: Order;
  createProduct: Product;
  createUser: User;
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

export type Order = {
  __typename?: 'Order';
  createdAt: Scalars['NaiveDateTime'];
  id: Scalars['Int'];
  status: Scalars['String'];
  total: Scalars['Int'];
  userId: Scalars['Int'];
};

export type OrderInput = {
  id: Scalars['Int'];
  status: Scalars['String'];
  total: Scalars['Int'];
  userId: Scalars['Int'];
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
  listOrder: Array<Order>;
  listProduct: Array<Product>;
  listUser: Array<User>;
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

export type Get_Menu_ListQueryVariables = Exact<{ [key: string]: never; }>;


export type Get_Menu_ListQuery = { __typename?: 'Query', listProduct: Array<{ __typename?: 'Product', id: number, name: string, price: number, stock: number }> };
