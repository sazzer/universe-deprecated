import React, { useState } from 'react';
import { useTranslation } from "react-i18next";
import useFormal from "@kevinwolf/formal-web";
import * as yup from "yup";

export interface StartLoginFormProps {
  onSubmit: (username: string, known: boolean) => void,
}
/**
 * Form rendered to collect the username to log in as.
 */
export const StartLoginForm: React.FC<StartLoginFormProps> = ({ onSubmit }) => {
  const { t } = useTranslation();

  const [pending, setPending] = useState(false);

  const schema = yup.object().shape({
    username: yup.string().required(t('login.username.errors.required')),
  });
  const formal = useFormal({ username: '' }, {
    schema,
    onSubmit: ({ username }) => {
      setPending(true);
      setTimeout(() => {
        setPending(false);
        onSubmit(username, true);
      }, 5000);
    }
  });

  return (
    <>
      <h3>{t('login.start.title')}</h3>
      <form {...formal.getFormProps()} data-test="startLoginForm">
        <div className="form-group" data-test="username">
          <label htmlFor="login_username">{t('login.username.label')}</label>
          <input type="text"
            className={formal.errors.username ? 'form-control is-invalid' : 'form-control'}
            id="login_username"
            autoFocus
            {...formal.getFieldProps('username')} />
          {formal.errors.username && <div className="invalid-feedback">{formal.errors.username}</div>}
        </div>
        <button {...formal.getSubmitButtonProps()}
          type="submit"
          className="btn btn-primary"
          disabled={pending || formal.getSubmitButtonProps().disabled}>
          {pending && <span className="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>}
          {t('login.start.submit')}
        </button>
      </form>
    </>
  );
};
