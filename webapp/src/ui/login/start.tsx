import * as yup from "yup";

import { ErrorMessage, FieldValues, useForm } from "react-hook-form";
import React, { useState } from "react";

import debug from "debug";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOG = debug("universe:ui:login:start");

/**
 * Page for starting the login process, allowing for input of the username
 */
export const StartLogin: React.FC = () => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);

  const { register, errors, handleSubmit } = useForm({
    validationSchema: yup.object().shape({
      username: yup
        .string()
        .required(
          t("login.username.errors.tag:universe,2020:validation-errors/missing")
        )
        .trim()
    }),
    validateCriteriaMode: "all",
    defaultValues: {
      username: ""
    }
  });

  const onSubmitHandler = async (data: FieldValues) => {
    LOG("Submitting form: %o", data);
    setLoading(true);
  };

  let errorMessage = <></>;

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
            disabled={loading}
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="username"
            as={<div className="invalid-feedback" />}
          />
        </div>

        <div className="form-group">
          <button type="submit" className="btn btn-primary" disabled={loading}>
            {loading && (
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
