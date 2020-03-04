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
    <div className="form-group">
      <div className="alert alert-danger" role="alert">
        {t("errors.unexpected", {
          message
        })}
      </div>
    </div>
  );
};
