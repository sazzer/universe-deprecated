import React from "react";
import { useTranslation } from "react-i18next";
import { useUser } from "../users";

/**
 * Wrapper around other components to only render if there is a currently logged in user
 */
export const LoggedIn: React.FC = ({ children }) => {
  const { hasUser } = useUser();
  const { t } = useTranslation();

  const body = hasUser ? children : t("errors.unauthenticated");

  return <>{body}</>;
};
