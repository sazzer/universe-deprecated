interface AccessToken {
  accessToken: string;
  expires: string;
}

type State = {
  userId: string | null;
  accessToken: AccessToken | null;
};

export const state: State = {
  userId: null,
  accessToken: null
};
