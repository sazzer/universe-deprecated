import React from "react";

/**
 * Props for the Authenticate User page
 */
export interface AuthenticateUserPageProps {
  username: string;
  onCancel: () => void;
}

/**
 * Page for authenticating an existing user
 */
export const AuthenticateUserPage: React.FC<AuthenticateUserPageProps> = ({
  username,
  onCancel
}) => {
  return <>{username}</>;
};
