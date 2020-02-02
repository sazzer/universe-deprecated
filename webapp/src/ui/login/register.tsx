import React, { useState } from 'react';
import { useTranslation } from "react-i18next";
import useFormal from "@kevinwolf/formal-web";
import * as yup from "yup";

/** Shape of the properties required for the Register Form view */
export interface RegisterFormProps {
  username: string,
  onSubmit: (username: string) => Promise<void>,
  onCancel: () => void,
}

/**
 * Render the view for the Register Form
 */
export const RegisterForm: React.FC<RegisterFormProps> = ({ onSubmit, onCancel, username }) => {
  const { t } = useTranslation();
  const [pending, setPending] = useState(false);
  const [error, setError] = useState('');

  const schema = yup.object().shape({
    username: yup.string()
      .required(t('login.username.errors.required'))
      .trim(),
    email: yup.string()
      .email(t('login.email.errors.email'))
      .required(t('login.email.errors.required'))
      .trim(),
    displayName: yup.string()
      .required(t('login.displayName.errors.required'))
      .trim(),
    password: yup.string()
      .required(t('login.password.errors.required'))
      .trim(),
    password2: yup.string()
      .required(t('login.password2.errors.required'))
      .when(['password'], (password: string, schema: any) => {
        return schema.oneOf([password], t('login.password2.errors.different'));
      })
      .trim(),
  });
  const formal = useFormal({ username: username, email: '', displayName: '', password: '', password2: '' }, {
    schema,
    onSubmit: async ({ username, email, displayName, password, password2 }) => {
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
      <h3>{t('login.register.title')}</h3>
      <form {...formal.getFormProps()} data-test="registerForm">
        <div className="form-group" data-test="username">
          <label htmlFor="username">{t('login.username.label')}</label>
          <input type="text"
            className={formal.errors.username ? 'form-control is-invalid' : 'form-control'}
            id="username"
            readOnly
            {...formal.getFieldProps('username')} />
          {formal.errors.username && <div className="invalid-feedback">{formal.errors.username}</div>}
        </div>
        <div className="form-group" data-test="email">
          <label htmlFor="email">{t('login.email.label')}</label>
          <input type="text"
            className={formal.errors.email ? 'form-control is-invalid' : 'form-control'}
            id="email"
            autoFocus
            {...formal.getFieldProps('email')} />
          {formal.errors.email && <div className="invalid-feedback">{formal.errors.email}</div>}
        </div>
        <div className="form-group" data-test="displayName">
          <label htmlFor="displayName">{t('login.displayName.label')}</label>
          <input type="text"
            className={formal.errors.displayName ? 'form-control is-invalid' : 'form-control'}
            id="displayName"
            {...formal.getFieldProps('displayName')} />
          {formal.errors.displayName && <div className="invalid-feedback">{formal.errors.displayName}</div>}
        </div>
        <div className="form-group" data-test="password">
          <label htmlFor="password">{t('login.password.label')}</label>
          <input type="password"
            className={formal.errors.password ? 'form-control is-invalid' : 'form-control'}
            id="password"
            {...formal.getFieldProps('password')} />
          {formal.errors.password && <div className="invalid-feedback">{formal.errors.password}</div>}
        </div>
        <div className="form-group" data-test="password2">
          <label htmlFor="password2">{t('login.password2.label')}</label>
          <input type="password"
            className={formal.errors.password2 ? 'form-control is-invalid' : 'form-control'}
            id="password2"
            {...formal.getFieldProps('password2')} />
          {formal.errors.password2 && <div className="invalid-feedback">{formal.errors.password2}</div>}
        </div>
        <div className="form-group">
          <button type="submit"
            className="btn btn-primary"
            disabled={pending}>
            {pending && <span className="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>}
            {t('login.register.submit')}
          </button>
          <button type="button"
            className="btn btn-link"
            disabled={pending}
            onClick={onCancel}>
            {t('login.register.cancel')}
          </button>
        </div>
        {errorMessage}
      </form>
    </>
  );
};
