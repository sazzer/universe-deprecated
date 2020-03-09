import * as yup from "yup";

import { ErrorMessage, FieldValues, useForm } from "react-hook-form";
import React, { useState } from "react";
import { changePassword, useUser } from "../../users";

import { SubmitButton } from "../components/form/buttons";
import { UnexpectedError } from "../components/form/error";
import { ValidationErrors } from "../../api";
import debug from "debug";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOG = debug("universe:ui:profile:changePassword");

/**
 * The actual Change Password form to work with
 */
export const ChangePasswordArea: React.FC = () => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [globalError, setGlobalError] = useState<string | undefined>(undefined);
  const { user } = useUser();

  const { register, errors, handleSubmit, setError } = useForm({
    validationSchema: yup.object().shape({
      password: yup
        .string()
        .required(
          t(
            "profile.password.password.errors.tag:universe,2020:validation-errors/missing"
          )
        ),
      password2: yup
        .string()
        .required(
          t(
            "profile.password.password2.errors.tag:universe,2020:validation-errors/missing"
          )
        )
        .when(["password"], (password: string, schema: any) => {
          return schema.oneOf(
            [password],
            t(
              "profile.password.password2.errors.tag:universe,2020:validation-errors/password/invalid-password"
            )
          );
        })
    }),
    validateCriteriaMode: "all",
    defaultValues: {
      password: "",
      password2: ""
    }
  });

  const onSubmitHandler = async (data: FieldValues) => {
    LOG("Submitting form: %o", data);
    setGlobalError(undefined);
    setLoading(true);

    try {
      const saved = await changePassword(user?.id || "", data.password);
    } catch (e) {
      if (e instanceof ValidationErrors) {
        e.errors.forEach(error => {
          const message = t(
            `profile.profile.${error.field}.errors.${error.type}`
          );
          setError(error.field, error.type, message);
        });
      } else {
        setGlobalError(e.toString());
      }
    }
    setLoading(false);
  };

  return (
    <form
      onSubmit={handleSubmit(onSubmitHandler)}
      data-test="changePasswordForm"
    >
      <div className="form-group" data-test="password">
        <label htmlFor="password">{t("profile.password.password.label")}</label>
        <input
          type="password"
          className={
            errors.password ? "form-control is-invalid" : "form-control"
          }
          id="password"
          name="password"
          readOnly={loading}
          autoFocus
          ref={register}
        />
        <ErrorMessage
          errors={errors}
          name="password"
          as={<div className="invalid-feedback" />}
        />
      </div>
      <div className="form-group" data-test="password2">
        <label htmlFor="password2">
          {t("profile.password.password2.label")}
        </label>
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
          {t("profile.password.submit")}
        </SubmitButton>
      </div>
      {globalError && <UnexpectedError message={globalError} />}
    </form>
  );
};
