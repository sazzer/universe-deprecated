import { Link, useHistory } from "react-router-dom";

import React from "react";
import { setAccessToken } from "../../api";
import { useTranslation } from "react-i18next";
import { useUser } from "../../users";

/** Header menu for the current user actions when the user is logged in */
export const UserMenu: React.FC = () => {
  const { t } = useTranslation();
  const { user, clearUser } = useUser();
  const history = useHistory();

  const logout = () => {
    setAccessToken(undefined);
    clearUser();
    history.push("/login");
  };

  return (
    <li className="nav-item dropdown">
      <a
        className="nav-link dropdown-toggle"
        href="/"
        id="navbarDropdown"
        role="button"
        data-toggle="dropdown"
        aria-haspopup="true"
        aria-expanded="false"
      >
        {user?.displayName}
      </a>
      <div className="dropdown-menu" aria-labelledby="navbarDropdown">
        <Link to="/profile" className="dropdown-item">
          {t("header.userMenu.profile")}
        </Link>
        <div className="dropdown-divider"></div>
        <Link to="/login" className="dropdown-item" onClick={logout}>
          {t("header.userMenu.logout")}
        </Link>
      </div>
    </li>
  );
};
