export interface User {
  userId: string;
  username: string;
  displayName: string;
  email: string | null;
}

type State = {
  users: User[];
};

export const state: State = {
  users: []
};
