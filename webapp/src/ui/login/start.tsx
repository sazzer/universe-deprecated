import React, { useState } from 'react';
import { useTranslation } from "react-i18next";
import useFormal from "@kevinwolf/formal-web";
import * as yup from "yup";

/** Shape of the properties required for the Start Login Form view */
export interface StartLoginFormProps {
  onSubmit: (username: string) => Promise<void>,
}

/**
 * Render the view for the Start Login Form
 */
export const StartLoginForm: React.FC<StartLoginFormProps> = ({ onSubmit }) => {
  const { t } = useTranslation();
  const [pending, setPending] = useState(false);
  const [error, setError] = useState('');

  const schema = yup.object().shape({
    username: yup.string()
      .required(t('login.username.errors.required'))
      .trim(),
  });
  const formal = useFormal({ username: '' }, {
    schema,
    onSubmit: async ({ username }) => {
      setPending(true);
      setError('');
      try {
        await onSubmit(username.trim());
      } catch (e) {
        setPending(false);
        setError(e.toString());
      }
    },
  });

  let errorMessage;
  if (error) {
    errorMessage = (
      <div className="form-group">
        <div className="alert alert-danger" role="alert">
          {t('errors.unexpected', {
            message: error,
          })}
        </div>
      </div>
    );
  }
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
        <div className="form-group">
          <button type="submit"
            className="btn btn-primary"
            disabled={pending}>
            {pending && <span className="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>}
            {t('login.start.submit')}
          </button>
        </div>
        {errorMessage}
      </form>
    </>
  );
};
