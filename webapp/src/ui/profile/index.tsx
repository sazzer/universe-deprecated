import React from "react";
import { LoggedIn } from "../login/loggedIn";

/**
 * React Component represnting the user profile page
 */
export const ProfilePage: React.FC = () => {
  return (
    <LoggedIn>
      <div>Profile Page</div>
    </LoggedIn>
  );
};
