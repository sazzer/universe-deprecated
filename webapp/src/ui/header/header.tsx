import React from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { LoginLink } from "./login";
import { UserMenu } from "./usermenu";
import { useOvermind } from "../../overmind";

/**
 * Component to represent the header bar of the entire page
 */
export const Header: React.FC = () => {
  const { t } = useTranslation();
  const { state } = useOvermind();

  let userMenu;
  if (state.authentication.isLoggedIn) {
    userMenu = <UserMenu />;
  } else {
    userMenu = <LoginLink />;
  }
  return (
    <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
      <Link to="/" className="navbar-brand">
        {t("page.title")}
      </Link>
      <button
        className="navbar-toggler"
        type="button"
        data-toggle="collapse"
        data-target="#navbarSupportedContent"
        aria-controls="navbarSupportedContent"
        aria-expanded="false"
        aria-label={t("header.toggleNavigation")}
      >
        <span className="navbar-toggler-icon"></span>
      </button>

      <div className="collapse navbar-collapse" id="navbarSupportedContent">
        <ul className="navbar-nav ml-auto">{userMenu}</ul>
      </div>
    </nav>
  );
};
