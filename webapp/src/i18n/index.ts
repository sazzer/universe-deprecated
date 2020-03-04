import LanguageDetector from "i18next-browser-languagedetector";
import defaultTranslations from "./messages.json";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources: {
      dev: {
        translation: defaultTranslations
      }
    },

    nsSeparator: false,

    debug: false,

    interpolation: {
      escapeValue: false
    },

    parseMissingKeyHandler: key => {
      if (process.env.NODE_ENV === "test") {
        throw new Error(`Missing message key: ${key}`);
      } else {
        return `!!${key}!!`;
      }
    }
  });

export default i18n;
