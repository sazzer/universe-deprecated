import React from "react";
import { useTranslation } from "react-i18next";

/**
 * The main entrypoint into the application
 */
export const App: React.FC = () => {
  const { t } = useTranslation();

  return <div>{t("page.title")}</div>;
};
