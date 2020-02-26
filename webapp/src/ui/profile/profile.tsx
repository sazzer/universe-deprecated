import React, { useEffect } from "react";
import { useForm, ErrorMessage, FieldValues } from "react-hook-form";
import { useTranslation } from "react-i18next";
import * as yup from "yup";
import { useOvermind } from "../../overmind";
import { Loader } from "../loader";
import { User, UserState } from "../../overmind/users/model";

/**
 * Props for rendering the User Profile Form
 */
interface UserProfileFormProps {
  user: User;
  userState: UserState;
}

/**
 * React component to actually render the user profile form
 */
export const UserProfileForm: React.FC<UserProfileFormProps> = ({ user }) => {
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
            "login.email.errors.tag:universe,2020:users/validation-errors/email/malformed"
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
            "login.displayName.errors.tag:universe,2020:validation-errors/missing"
          )
        )
        .trim()
    }),
    validateCriteriaMode: "all",
    defaultValues: {
      username: user.username,
      email: user.email || "",
      displayName: user.displayName
    }
  });

  const onSubmitHandler = async (data: FieldValues) => {
    console.log(data);
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
 * React Component to represent the user profile area of the profile page
 */
export const UserProfileArea: React.FC = () => {
  const { state, actions } = useOvermind();

  const currentUser = state.authentication.userId;

  useEffect(() => {
    if (currentUser !== null) {
      actions.users.fetchUser(currentUser);
    }
  }, [currentUser, actions.users]);

  const storedUser = state.users.userById(currentUser || "");
  const user = storedUser.user;

  if (storedUser.state === "LOADING" || user === undefined) {
    return <Loader />;
  } else {
    return <UserProfileForm userState={storedUser.state} user={user} />;
  }
};
