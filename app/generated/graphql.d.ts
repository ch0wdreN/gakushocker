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
};

export type MenuInput = {
  menu: Scalars['String'];
  price: Scalars['Int'];
  stock: Scalars['Int'];
};

export type MenuRecord = {
  __typename?: 'MenuRecord';
  id: Scalars['Int'];
  menu: Scalars['String'];
  price: Scalars['Int'];
  stock: Scalars['Int'];
};

export type Mutation = {
  __typename?: 'Mutation';
  createMenu: MenuRecord;
  createOrder: Order;
};


export type MutationCreateMenuArgs = {
  input?: InputMaybe<MenuInput>;
};


export type MutationCreateOrderArgs = {
  input?: InputMaybe<OrderInput>;
};

export type Order = {
  __typename?: 'Order';
  id: Scalars['Int'];
  menu: Scalars['String'];
  orderedAt: Scalars['String'];
  price: Scalars['Int'];
};

export type OrderInput = {
  menu: Scalars['String'];
  price: Scalars['Int'];
};

export type Query = {
  __typename?: 'Query';
  getMenu: MenuRecord;
  getOrder: Order;
  listMenu: Array<MenuRecord>;
  listOrder: Array<Order>;
};

export type Get_Menu_ListQueryVariables = Exact<{ [key: string]: never; }>;


export type Get_Menu_ListQuery = { __typename?: 'Query', listMenu: Array<{ __typename?: 'MenuRecord', id: number, menu: string, price: number, stock: number }> };
