import * as yup from "yup";

import { ErrorMessage, FieldValues, useForm } from "react-hook-form";
import React, { useState } from "react";

import { SubmitButton } from "../components/form/buttons";
import { UnexpectedError } from "../components/form/error";
import { checkUsername } from "../../users";
import debug from "debug";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOG = debug("universe:ui:login:start");

/**
 * Props required for the the StartLogin page
 */
export interface StartLoginPageProps {
  /** Callback when a username has been submitted */
  onUsername: (username: string, known: boolean) => void;
}

/**
 * Page for starting the login process, allowing for input of the username
 */
export const StartLoginPage: React.FC<StartLoginPageProps> = ({
  onUsername
}) => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);

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
    setError(undefined);
    setLoading(true);

    try {
      const result = await checkUsername(data.username);
      onUsername(data.username, result);
    } catch (e) {
      setLoading(false);
      setError(e.toString());
    }
  };

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
            readOnly={loading}
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="username"
            as={<div className="invalid-feedback" />}
          />
        </div>

        <div className="form-group">
          <SubmitButton loading={loading}>
            {t("login.start.submit")}
          </SubmitButton>
        </div>
        {error && <UnexpectedError message={error} />}
      </form>
    </>
  );
};
