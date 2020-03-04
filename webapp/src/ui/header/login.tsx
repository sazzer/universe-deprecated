import { Link } from "react-router-dom";
import React from "react";
import { useTranslation } from "react-i18next";

/** Header link to allow the user to log in */
export const LoginLink: React.FC = () => {
  const { t } = useTranslation();

  return (
    <li className="nav-item">
      <Link to="/login" className="nav-link">
        {t("header.loginMenu.title")}
      </Link>
    </li>
  );
};
