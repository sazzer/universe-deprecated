import React from "react";
import { useTranslation } from "react-i18next";

/**
 * Loader to display whilst the page is still loading
 */
export const Loader: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="d-flex justify-content-center">
      <div className="spinner-border" role="status">
        <span className="sr-only">t('loader.label')</span>
      </div>
    </div>
  );
};
