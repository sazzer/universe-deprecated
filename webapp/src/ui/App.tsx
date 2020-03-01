import React from "react";
import { BrowserRouter as Router } from "react-router-dom";
import { useTranslation } from "react-i18next";

/**
 * The main entrypoint into the application
 */
export const App: React.FC = () => {
  const { t } = useTranslation();

  return (
    <Router>
      <div>{t("page.title")}</div>
    </Router>
  );
};
