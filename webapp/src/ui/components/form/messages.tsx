import React from "react";

/**
 * Props for the Messages component
 */
export interface MessagesProps {
  type: "primary" | "secondary" | "success" | "danger" | "warning" | "info";
}

/**
 * Message to display in a form when something happened
 */
export const Message: React.FC<MessagesProps> = ({ children, type }) => {
  return (
    <div className="form-group">
      <div className={`alert alert-${type}`} role="alert">
        {children}
      </div>
    </div>
  );
};
