import { atom } from 'recoil';
import { User } from '../../generated/graphql';

export const userState = atom<User>({
  key: 'userState',
  default: {
    id: 0,
    displayName: '',
    email: '',
    password: '',
    point: 0,
  },
});
// TODO: ユーザーの状態管理をどうするかを考える
//       バックエンドからはユーザーの更新された情報はあんまり返ってこないからバックエンドに新しいの
//       を追加して更新される処理が行われた時更新後のユーザー情報を取れるようにする？
//       tokenの期限が切れてなければユーザー情報を取れるようにしたい
