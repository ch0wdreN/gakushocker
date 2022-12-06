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

export type Customer = {
  __typename?: 'Customer';
  id: Scalars['Int'];
  menu: Scalars['String'];
  orderedAt: Scalars['String'];
  price: Scalars['Int'];
};

export type CustomerInput = {
  menu: Scalars['String'];
  price: Scalars['Int'];
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
  addNewMenu: MenuRecord;
  addNewOrder: Customer;
};


export type MutationAddNewMenuArgs = {
  input?: InputMaybe<MenuInput>;
};


export type MutationAddNewOrderArgs = {
  input?: InputMaybe<CustomerInput>;
};

export type Query = {
  __typename?: 'Query';
  getAllMenu: Array<MenuRecord>;
  getAllOrder: Array<Customer>;
  getMenuById: MenuRecord;
  getOrderById: Customer;
};

export type Get_All_MenuQueryVariables = Exact<{ [key: string]: never; }>;


export type Get_All_MenuQuery = { __typename?: 'Query', getAllMenu: Array<{ __typename?: 'MenuRecord', id: number, menu: string, price: number, stock: number }> };
