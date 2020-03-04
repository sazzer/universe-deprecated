import React from "react";

/**
 * Props for the SubmitButton
 */
export interface SubmitButtonProps {
  loading: boolean;
}

/**
 * Submit Button, displaying some provided content and including a spinner when the loading flag is set
 */
export const SubmitButton: React.FC<SubmitButtonProps> = ({
  loading,
  children
}) => {
  return (
    <button type="submit" className="btn btn-primary" disabled={loading}>
      {loading && (
        <>
          <span
            className="spinner-border spinner-border-sm"
            role="status"
            aria-hidden="true"
          ></span>
          &nbsp;
        </>
      )}
      {children}
    </button>
  );
};

/**
 * Props for the CancelButton
 */
export interface CancelButtonProps {
  disabled: boolean;
  onClick: () => void;
}

/**
 * Cancel button, displaying some content as a link
 */
export const CancelButton: React.FC<CancelButtonProps> = ({
  disabled,
  onClick,
  children
}) => {
  return (
    <button
      type="button"
      className="btn btn-link"
      disabled={disabled}
      onClick={onClick}
    >
      {children}
    </button>
  );
};
