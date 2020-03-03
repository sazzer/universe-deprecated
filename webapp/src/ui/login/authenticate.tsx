import * as yup from "yup";

import { CancelButton, SubmitButton } from "../components/form/buttons";
import { ErrorMessage, FieldValues, useForm } from "react-hook-form";
import React, { useState } from "react";

import { UnexpectedError } from "../components/form/error";
import debug from "debug";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOG = debug("universe:ui:login:authenticate");

/**
 * Props for the Authenticate User page
 */
export interface AuthenticateUserPageProps {
  username: string;
  onCancel: () => void;
}

/**
 * Page for authenticating an existing user
 */
export const AuthenticateUserPage: React.FC<AuthenticateUserPageProps> = ({
  username,
  onCancel
}) => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [globalError, setGlobalError] = useState<string | undefined>(undefined);

  const { register, errors, handleSubmit } = useForm({
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
      username: username,
      password: ""
    }
  });

  const onSubmitHandler = async (data: FieldValues) => {
    LOG("Submitting form: %o", data);
    setGlobalError(undefined);
    setLoading(true);
  };

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
            readOnly={loading}
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="password"
            as={<div className="invalid-feedback" />}
          />
        </div>
        <div className="form-group">
          <SubmitButton loading={loading}>
            {t("login.authenticate.submit")}
          </SubmitButton>
          <CancelButton disabled={loading} onClick={onCancel}>
            {t("login.authenticate.cancel")}
          </CancelButton>
        </div>
        {globalError && <UnexpectedError message={globalError} />}
      </form>
    </>
  );
};
