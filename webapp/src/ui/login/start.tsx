import React from 'react';
import { useTranslation } from "react-i18next";

export const StartLoginForm: React.FC = () => {
  const { t } = useTranslation();
  return (
    <>
      <h3>{t('login.start.title')}</h3>
      <form action="/login" method="POST">
        <div className="form-group" data-test="username">
          <label htmlFor="login_username">{t('login.username.label')}</label>
          <input type="text" className="form-control" id="login_username" name="username" autoFocus required />
        </div>
        <button type="submit" className="btn btn-primary">{t('login.start.submit')}</button>
      </form>
    </>
  );
};
