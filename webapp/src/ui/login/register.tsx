import React from "react";
import { useTranslation } from "react-i18next";
import { useForm, ErrorMessage, FieldValues } from "react-hook-form";
import * as yup from "yup";
import { useOvermind } from "../../overmind";

/**
 * Render the view for the Register Form
 */
export const RegisterForm: React.FC = () => {
  const { t } = useTranslation();
  const { state, actions } = useOvermind();

  const onSubmitHandler = async (data: FieldValues) => {
    await actions.login.register({
      username: data.username,
      email: data.email,
      displayName: data.displayName,
      password: data.password
    });
  };

  const { register, errors, handleSubmit } = useForm({
    validationSchema: yup.object().shape({
      username: yup
        .string()
        .required(t("login.username.errors.required"))
        .trim(),
      email: yup
        .string()
        .email(t("login.email.errors.email"))
        .required(t("login.email.errors.required"))
        .trim(),
      displayName: yup
        .string()
        .required(t("login.displayName.errors.required"))
        .trim(),
      password: yup.string().required(t("login.password.errors.required")),
      password2: yup
        .string()
        .required(t("login.password2.errors.required"))
        .when(["password"], (password: string, schema: any) => {
          return schema.oneOf(
            [password],
            t("login.password2.errors.different")
          );
        })
    }),
    validateCriteriaMode: "all",
    defaultValues: {
      username: state.login.username || "",
      email: "",
      displayName: "",
      password: "",
      password2: ""
    }
  });

  let errorMessage;

  return (
    <>
      <h3>{t("login.register.title")}</h3>
      <form onSubmit={handleSubmit(onSubmitHandler)} data-test="registerForm">
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
        <div className="form-group" data-test="email">
          <label htmlFor="email">{t("login.email.label")}</label>
          <input
            type="text"
            className={
              errors.email ? "form-control is-invalid" : "form-control"
            }
            id="email"
            name="email"
            autoFocus
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="email"
            as={<div className="invalid-feedback" />}
          />
        </div>
        <div className="form-group" data-test="displayName">
          <label htmlFor="displayName">{t("login.displayName.label")}</label>
          <input
            type="text"
            className={
              errors.displayName ? "form-control is-invalid" : "form-control"
            }
            id="displayName"
            name="displayName"
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="displayName"
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
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="password"
            as={<div className="invalid-feedback" />}
          />
        </div>
        <div className="form-group" data-test="password2">
          <label htmlFor="password2">{t("login.password2.label")}</label>
          <input
            type="password"
            className={
              errors.password2 ? "form-control is-invalid" : "form-control"
            }
            id="password2"
            name="password2"
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="password2"
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
            {t("login.register.submit")}
          </button>
          <button
            type="button"
            className="btn btn-link"
            disabled={state.login.loading}
            onClick={actions.login.cancelLogin}
          >
            {t("login.register.cancel")}
          </button>
        </div>
        {errorMessage}
      </form>
    </>
  );
};
