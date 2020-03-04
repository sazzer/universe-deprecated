import * as yup from "yup";

import { CancelButton, SubmitButton } from "../components/form/buttons";
import { ErrorMessage, FieldValues, useForm } from "react-hook-form";
import React, { useState } from "react";
import { register as registerUser, useUser } from "../../users";

import { UnexpectedError } from "../components/form/error";
import { ValidationErrors } from "../../api";
import debug from "debug";
import { useHistory } from "react-router-dom";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOG = debug("universe:ui:login:register");

/**
 * Props for the Register User page
 */
export interface RegisterUserPageProps {
  username: string;
  onCancel: () => void;
}

/**
 * Page for registering a new user
 */
export const RegisterUserPage: React.FC<RegisterUserPageProps> = ({
  username,
  onCancel
}) => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [globalError, setGlobalError] = useState<string | undefined>(undefined);
  const history = useHistory();
  const { storeUser } = useUser();

  const { register, errors, handleSubmit, setError } = useForm({
    validationSchema: yup.object().shape({
      username: yup
        .string()
        .required(
          t("login.username.errors.tag:universe,2020:validation-errors/missing")
        )
        .trim(),
      email: yup
        .string()
        .email(
          t(
            "login.email.errors.tag:universe,2020:users/validation-errors/email/malformed"
          )
        )
        .required(
          t("login.email.errors.tag:universe,2020:validation-errors/missing")
        )
        .trim(),
      displayName: yup
        .string()
        .required(
          t(
            "login.displayName.errors.tag:universe,2020:validation-errors/missing"
          )
        )
        .trim(),
      password: yup
        .string()
        .required(
          t("login.password.errors.tag:universe,2020:validation-errors/missing")
        ),
      password2: yup
        .string()
        .required(
          t(
            "login.password2.errors.tag:universe,2020:validation-errors/missing"
          )
        )
        .when(["password"], (password: string, schema: any) => {
          return schema.oneOf(
            [password],
            t(
              "login.password2.errors.tag:universe,2020:validation-errors/password/invalid-password"
            )
          );
        })
    }),
    validateCriteriaMode: "all",
    defaultValues: {
      username: username,
      email: "",
      displayName: "",
      password: "",
      password2: ""
    }
  });

  const onSubmitHandler = async (data: FieldValues) => {
    LOG("Submitting form: %o", data);
    setGlobalError(undefined);
    setLoading(true);

    try {
      const user = await registerUser(
        data.username,
        data.email,
        data.displayName,
        data.password
      );
      storeUser(user);
      history.push("/profile");
    } catch (e) {
      if (e instanceof ValidationErrors) {
        e.errors.forEach(error => {
          const message = t(`login.${error.field}.errors.${error.type}`);
          setError(error.field, error.type, message);
        });
      } else {
        setGlobalError(e.toString());
      }
      setLoading(false);
    }
  };

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
            readOnly={loading}
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
            readOnly={loading}
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
            readOnly={loading}
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
            readOnly={loading}
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="password2"
            as={<div className="invalid-feedback" />}
          />
        </div>
        <div className="form-group">
          <SubmitButton loading={loading}>
            {t("login.register.submit")}
          </SubmitButton>
          <CancelButton disabled={loading} onClick={onCancel}>
            {t("login.register.cancel")}
          </CancelButton>
        </div>
        {globalError && <UnexpectedError message={globalError} />}
      </form>
    </>
  );
};
