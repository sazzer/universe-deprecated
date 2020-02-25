import React from "react";
import { useOvermind } from "../../overmind";

/**
 * React Component to render the children only when there is an active logged-in session
 */
export const LoggedIn: React.FC = ({ children }) => {
  const { state } = useOvermind();

  if (
    state.authentication.isLoggedIn ||
    process.env.NODE_ENV === "development"
  ) {
    return <>{children}</>;
  } else {
    return <div>You must be logged in for this</div>;
  }
};
