import { Message } from "./messages";
import React from "react";
import { useTranslation } from "react-i18next";

/**
 * Props for the UnexpectedError component
 */
export interface UnexpectedErrorProps {
  message: string;
}

/**
 * Error Message to display in a form when something unexpectedly goes wrong
 */
export const UnexpectedError: React.FC<UnexpectedErrorProps> = ({
  message
}) => {
  const { t } = useTranslation();

  return (
    <Message type="danger">
      {t("errors.unexpected", {
        message
      })}
    </Message>
  );
};
