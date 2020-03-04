import { HomePage } from "./homePage";
import React from "react";
import { render } from "@testing-library/react";

test("It renders correctly", () => {
  const { container } = render(<HomePage />);
  expect(container).toMatchSnapshot();
});
