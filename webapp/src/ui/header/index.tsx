import { Link } from "react-router-dom";
import { LoginLink } from "./login";
import React from "react";
import { useTranslation } from "react-i18next";
import { useUser } from "../../users";

/**
 * The header bar for the application
 */
export const HeaderBar: React.FC = () => {
  const { t } = useTranslation();
  const { hasUser } = useUser();

  let userMenu;
  if (hasUser) {
    userMenu = "Hello";
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
