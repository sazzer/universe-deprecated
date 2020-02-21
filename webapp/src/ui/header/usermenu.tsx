import React from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { useOvermind } from "../../overmind";

/** Header menu for the current user actions when the user is logged in */
export const UserMenu: React.FC = () => {
  const { t } = useTranslation();
  const { state } = useOvermind();

  let username;
  const currentUser =
    state.authentication.userId !== null
      ? state.users.users[state.authentication.userId]
      : undefined;
  if (currentUser !== undefined) {
    username = currentUser.displayName;
  }

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
        {username}
      </a>
      <div className="dropdown-menu" aria-labelledby="navbarDropdown">
        <Link to="/profile" className="dropdown-item">
          {t("header.userMenu.profile")}
        </Link>
        <div className="dropdown-divider"></div>
        <a className="dropdown-item" href="/">
          {t("header.userMenu.logout")}
        </a>
      </div>
    </li>
  );
};
