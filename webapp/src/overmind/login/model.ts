import { User } from "../users/model";

/** The shape of an access token */
export interface AccessToken {
  /** The access token */
  token: string;
  /** The expiry date */
  expiry: string;
}

/** The shape of an authenticated user */
export interface AuthenticatedUser extends User {
  /** The access token to use for this user */
  accessToken: AccessToken;
}
