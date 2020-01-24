import React from 'react';
import { useTranslation } from "react-i18next";
import { Link } from 'react-router-dom';

export const Header: React.FC = () => {
  const { t } = useTranslation();

  return (
    <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
      <Link to="/" className="navbar-brand">{t('page.title')}</Link>
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
            <Link to="/login" className="nav-link">{t('header.loginMenu.title')}</Link>
          </li>
        </ul>
      </div>
    </nav >
  );
};
