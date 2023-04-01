import { User } from '../../generated/graphql';

export interface ReservedUser {
  id: number;
  display_name: string;
  email: string;
  password: string;
  point: number;
}

export const toCamel = (reservedUser: ReservedUser) => {
  let user: User = {
    displayName: reservedUser.display_name,
    ...reservedUser,
  };
  return user;
};
