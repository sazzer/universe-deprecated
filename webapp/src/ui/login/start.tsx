import React, { useState } from 'react';
import { useTranslation } from "react-i18next";
import { useForm, ErrorMessage, FieldValues } from 'react-hook-form';
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

  const { register, errors, handleSubmit } = useForm({
    validationSchema: yup.object().shape({
      username: yup.string()
        .required(t('login.username.errors.required'))
        .trim(),
    }),
    validateCriteriaMode: 'all',
    defaultValues: {
      username: ''
    }
  });

  const onSubmitHandler = async (data: FieldValues) => {
    setPending(true);
    setError('');
    try {
      await onSubmit(data.username.trim());
    } catch (e) {
      setPending(false);
      setError(e.toString());
    }
  };

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
      <form onSubmit={handleSubmit(onSubmitHandler)} data-test="startLoginForm">
        <div className="form-group" data-test="username">
          <label htmlFor="username">{t('login.username.label')}</label>
          <input type="text"
            className={errors.username ? 'form-control is-invalid' : 'form-control'}
            id="username"
            name="username"
            autoFocus
            ref={register} />
          <ErrorMessage errors={errors} name="username" as={<div className="invalid-feedback" />} />
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
