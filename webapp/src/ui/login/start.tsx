import React from "react";
import { useTranslation } from "react-i18next";
import { useForm, ErrorMessage, FieldValues } from "react-hook-form";
import * as yup from "yup";
import { useOvermind } from "../../overmind";

/**
 * Render the view for the Start Login Form
 */
export const StartLoginForm: React.FC = () => {
  const { t } = useTranslation();
  const { state, actions } = useOvermind();

  const { register, errors, handleSubmit } = useForm({
    validationSchema: yup.object().shape({
      username: yup
        .string()
        .required(t("login.username.errors.required"))
        .trim()
    }),
    validateCriteriaMode: "all",
    defaultValues: {
      username: ""
    }
  });

  const onSubmitHandler = async (data: FieldValues) => {
    actions.login.checkUsername(data.username);
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
      <h3>{t("login.start.title")}</h3>
      <form onSubmit={handleSubmit(onSubmitHandler)} data-test="startLoginForm">
        <div className="form-group" data-test="username">
          <label htmlFor="username">{t("login.username.label")}</label>
          <input
            type="text"
            className={
              errors.username ? "form-control is-invalid" : "form-control"
            }
            id="username"
            name="username"
            autoFocus
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="username"
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
            {t("login.start.submit")}
          </button>
        </div>
        {errorMessage}
      </form>
    </>
  );
};
