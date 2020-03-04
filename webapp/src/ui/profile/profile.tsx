import * as yup from "yup";

import { ErrorMessage, FieldValues, useForm } from "react-hook-form";
import React, { useEffect, useState } from "react";
import { User, getUserById, useUser } from "../../users";

import { Loader } from "../components/loader";
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

  const { register, errors, handleSubmit } = useForm({
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
            "profile.email.errors.tag:universe,2020:users/validation-errors/email/malformed"
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
            "profile.displayName.errors.tag:universe,2020:validation-errors/missing"
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
    LOG("Updating user profile: %o", data);
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
            ref={register}
          />
          <ErrorMessage
            errors={errors}
            name="displayName"
            as={<div className="invalid-feedback" />}
          />
        </div>
        <div className="form-group">
          <button type="submit" className="btn btn-primary">
            {t("profile.profile.submit")}
          </button>
        </div>
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
