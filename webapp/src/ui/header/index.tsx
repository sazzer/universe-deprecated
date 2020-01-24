import React from 'react';
import { useTranslation } from "react-i18next";

export const Header: React.FC = () => {
  const { t } = useTranslation();

  return (
    <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
      <a className="navbar-brand" href="/">{t('page.title')}</a>
      <button className="navbar-toggler"
        type="button"
        data-toggle="collapse"
        data-target="#navbarSupportedContent"
        aria-controls="navbarSupportedContent"
        aria-expanded="false"
        aria-label="{t('header.toggleNavigation')}">
        <span className="navbar-toggler-icon"></span>
      </button>

      <div className="collapse navbar-collapse" id="navbarSupportedContent">
        <ul className="navbar-nav ml-auto">
          <li className="nav-item">
            <a className="nav-link" href="/login">{t('header.loginMenu.title')}</a>
          </li>
        </ul>
      </div>
    </nav>
  );
};
