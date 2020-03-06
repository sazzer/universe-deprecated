import React, { createContext, useContext, useState } from "react";

import { User } from "./model";

/** Type representing the current state of the user context */
export type UserState = User | null;

/** The shape of the contet available through this */
export type UserContextState = {
  user: UserState;
  hasUser: boolean;
  storeUser: (user: User) => void;
  clearUser: () => void;
};

/** The React context for storing the current user */
export const UserContext = createContext<UserContextState>({
  user: null,
  hasUser: false,
  storeUser: () => {},
  clearUser: () => {}
});

/**
 * React Component to wrap the application in something that understands the storage of the current user
 */
export const UserProvider: React.FC = ({ children }) => {
  const [user, setUser] = useState<UserState>(null);

  const contextState: UserContextState = {
    user,
    storeUser: (user: User) => setUser(user),
    clearUser: () => setUser(null),
    hasUser: user !== null
  };

  return (
    <UserContext.Provider value={contextState}>{children}</UserContext.Provider>
  );
};

/**
 * React Hook to make user details available to the rest of the code
 */
export const useUser = () => {
  return useContext(UserContext);
};
