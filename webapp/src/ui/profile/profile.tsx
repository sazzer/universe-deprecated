import * as yup from "yup";

import { ErrorMessage, FieldValues, useForm } from "react-hook-form";
import React, { useEffect, useState } from "react";
import { User, getUserById, updateUserProfile, useUser } from "../../users";

import { Loader } from "../components/loader";
import { Message } from "../components/form/messages";
import { SubmitButton } from "../components/form/buttons";
import { UnexpectedError } from "../components/form/error";
import { ValidationErrors } from "../../api";
import debug from "debug";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOG = debug("universe:ui:profile:userProfile");

/** The props for the User Profile Form */
interface UserProfileFormProps {
  user: User | null;
}

/**
 * The actual User Profile form to work with
 */
const UserProfileForm: React.FC<UserProfileFormProps> = ({ user }) => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [globalError, setGlobalError] = useState<string | undefined>(undefined);
  const [success, setSuccess] = useState<boolean>(false);
  const { storeUser } = useUser();

  const { register, errors, handleSubmit, setError } = useForm({
    validationSchema: yup.object().shape({
      username: yup
        .string()
        .required(
          t(
            "profile.profile.username.errors.tag:universe,2020:validation-errors/missing"
          )
        )
        .trim(),
      email: yup
        .string()
        .email(
          t(
            "profile.profile.email.errors.tag:universe,2020:users/validation-errors/email/malformed"
          )
        )
        .required(
          t(
            "profile.profile.email.errors.tag:universe,2020:validation-errors/missing"
          )
        )
        .trim(),
      displayName: yup
        .string()
        .required(
          t(
            "profile.profile.displayName.errors.tag:universe,2020:validation-errors/missing"
          )
        )
        .trim()
    }),
    validateCriteriaMode: "all",
    defaultValues: {
      username: user?.username || "",
      email: user?.email || "",
      displayName: user?.displayName || ""
    }
  });

  const onSubmitHandler = async (data: FieldValues) => {
    LOG("Submitting form: %o", data);
    setGlobalError(undefined);
    setLoading(true);
    setSuccess(false);

    try {
      const saved = await updateUserProfile(
        user?.id || "",
        data.email,
        data.displayName
      );
      storeUser(saved);
      setSuccess(true);
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
    <>
      <form
        onSubmit={handleSubmit(onSubmitHandler)}
        data-test="userProfileForm"
      >
        <div className="form-group" data-test="username">
          <label htmlFor="username">
            {t("profile.profile.username.label")}
          </label>
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
          <label htmlFor="email">{t("profile.profile.email.label")}</label>
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
          <label htmlFor="displayName">
            {t("profile.profile.displayName.label")}
          </label>
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
        <div className="form-group">
          <SubmitButton loading={loading}>
            {t("profile.profile.submit")}
          </SubmitButton>
        </div>
        {globalError && <UnexpectedError message={globalError} />}
        {success && (
          <Message type="success">{t("profile.profile.success")}</Message>
        )}
      </form>
    </>
  );
};

/**
 * Area of the Profile Page to manage the User Profile
 */
export const UserProfileArea: React.FC = () => {
  const { user, storeUser } = useUser();
  const [loading, setLoading] = useState(true);

  const userId = user?.id;

  useEffect(() => {
    if (userId) {
      setLoading(true);
      getUserById(userId).then(loadedUser => {
        storeUser(loadedUser);
        setLoading(false);
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [userId]);

  return (
    <Loader loading={loading}>
      <UserProfileForm user={user} />
    </Loader>
  );
};
