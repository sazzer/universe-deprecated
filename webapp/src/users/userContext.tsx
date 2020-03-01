import React, { createContext, useContext, useState } from "react";

import { User } from "./model";

/** Type representing the current state of the user context */
type UserState = User | null;

/** The shape of the contet available through this */
type UserContext = {
  user: UserState;
  hasUser: boolean;
  storeUser: (user: User) => void;
  clearUser: () => void;
};

/** The React context for storing the current user */
const UserContext = createContext<UserContext>({
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

  const contextState: UserContext = {
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
