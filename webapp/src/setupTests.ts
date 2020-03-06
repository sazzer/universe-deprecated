import "@testing-library/jest-dom/extend-expect";
import "./i18n";
import "mutationobserver-shim";

import { configure } from "@testing-library/react";

configure({ testIdAttribute: "data-test" });
