import React from "react";
import { useTranslation } from "react-i18next";
import { useForm, ErrorMessage, FieldValues } from "react-hook-form";
import * as yup from "yup";
import { useOvermind } from "../../overmind";
import { useHistory } from "react-router-dom";
import { AuthenticationError } from "../../login/effects";

/**
 * Render the view for the Authenticate Form
 */
export const AuthenticateForm: React.FC = () => {
  const { t } = useTranslation();
  const { state, actions } = useOvermind();
  const history = useHistory();

  const { register, errors, handleSubmit, setError } = useForm({
    validationSchema: yup.object().shape({
      username: yup
        .string()
        .required(
          t("login.username.errors.tag:universe,2020:validation-errors/missing")
        )
        .trim(),
      password: yup
        .string()
        .required(
          t("login.password.errors.tag:universe,2020:validation-errors/missing")
        )
    }),
    validateCriteriaMode: "all",
    defaultValues: {
      username: state.login.username || "",
      password: ""
    }
  });

  const onSubmitHandler = async (data: FieldValues) => {
    const result = await actions.login.authenticate({
      username: data.username,
      password: data.password
    });

    if (result instanceof AuthenticationError) {
      const message = t(
        "login.password.errors.tag:universe,2020:users/problems/login_failure"
      );
      setError(
        "password",
        "tag:universe,2020:users/problems/login_failure",
        message
      );
    } else if (result === true) {
      history.push("/profile");
    }
  };

  let errorMessage;
  if (state.login.error) {
    errorMessage = (
      <div className="form-group">
        <div className="alert alert-danger" role="alert">
          {t("errors.unexpected", {
            message: state.login.error
          })}
        </div>
      </div>
    );
  }

  return (
    <>
      <h3>{t("login.authenticate.title")}</h3>
      <form
        onSubmit={handleSubmit(onSubmitHandler)}
        data-test="authenticateForm"
      >
        <div className="form-group" data-test="username">
          <label htmlFor="username">{t("login.username.label")}</label>
          <input
            type="text"
            className={
              errors.username ? "form-control is-invalid" : "form-control"
            }
            id="username"
            name="username"
            readOnly
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="username"
            as={<div className="invalid-feedback" />}
          />
        </div>
        <div className="form-group" data-test="password">
          <label htmlFor="password">{t("login.password.label")}</label>
          <input
            type="password"
            className={
              errors.password ? "form-control is-invalid" : "form-control"
            }
            id="password"
            name="password"
            autoFocus
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="password"
            as={<div className="invalid-feedback" />}
          />
        </div>
        <div className="form-group">
          <button
            type="submit"
            className="btn btn-primary"
            disabled={state.login.loading}
          >
            {state.login.loading && (
              <span
                className="spinner-border spinner-border-sm"
                role="status"
                aria-hidden="true"
              ></span>
            )}
            {t("login.authenticate.submit")}
          </button>
          <button
            type="button"
            className="btn btn-link"
            disabled={state.login.loading}
            onClick={actions.login.cancelLogin}
          >
            {t("login.authenticate.cancel")}
          </button>
        </div>
        {errorMessage}
      </form>
    </>
  );
};
