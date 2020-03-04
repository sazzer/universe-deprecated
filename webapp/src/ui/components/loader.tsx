import React from "react";
import { useTranslation } from "react-i18next";

/** The props for the Loader component */
export interface LoaderProps {
  loading: boolean;
}

/**
 * Simple component to indicate that a page is still loading
 */
export const Loader: React.FC<LoaderProps> = ({ loading, children }) => {
  const { t } = useTranslation();

  if (loading) {
    return (
      <div className="d-flex justify-content-center">
        <div className="spinner-border" role="status">
          <span className="sr-only">{t("loader.label")}</span>
        </div>
      </div>
    );
  } else {
    return <>{children}</>;
  }
};
