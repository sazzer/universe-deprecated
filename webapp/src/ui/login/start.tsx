import React, { useState } from 'react';
import { useTranslation } from "react-i18next";
import useFormal from "@kevinwolf/formal-web";
import * as yup from "yup";

/** Shape of the properties required for the Start Login Form view */
export interface StartLoginFormViewProps {
  onSubmit: (username: string) => void,
  pending: boolean,
}

/**
 * Render the view for the Start Login Form
 */
export const StartLoginFormView: React.FC<StartLoginFormViewProps> = ({ onSubmit, pending }) => {
  const { t } = useTranslation();
  const schema = yup.object().shape({
    username: yup.string()
      .required(t('login.username.errors.required'))
      .matches(/\S+/, t('login.username.errors.required')),
  });
  const formal = useFormal({ username: '' }, {
    schema,
    onSubmit: ({ username }) => onSubmit(username.trim()),
  });

  return (
    <>
      <h3>{t('login.start.title')}</h3>
      <form {...formal.getFormProps()} data-test="startLoginForm">
        <div className="form-group" data-test="username">
          <label htmlFor="username">{t('login.username.label')}</label>
          <input type="text"
            className={formal.errors.username ? 'form-control is-invalid' : 'form-control'}
            id="username"
            autoFocus
            {...formal.getFieldProps('username')} />
          {formal.errors.username && <div className="invalid-feedback">{formal.errors.username}</div>}
        </div>
        <button type="submit"
          className="btn btn-primary"
          disabled={pending}>
          {pending && <span className="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>}
          {t('login.start.submit')}
        </button>
      </form>
    </>
  );
}

/** Shape of the properties for the Start Login Form */
export interface StartLoginFormProps {
  onSubmit: (username: string, known: boolean) => void,
}

/**
 * Form rendered to collect the username to log in as.
 */
export const StartLoginForm: React.FC<StartLoginFormProps> = ({ onSubmit }) => {
  const [pending, setPending] = useState(false);

  return <StartLoginFormView pending={pending} onSubmit={(username) => {
    setPending(true);
    setTimeout(() => {
      setPending(false);
      onSubmit(username, true);
    }, 2000);
  }} />;
};
