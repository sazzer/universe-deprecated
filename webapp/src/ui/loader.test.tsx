import React from "react";
import { render } from "@testing-library/react";
import { MemoryRouter as Router } from "react-router-dom";
import { Loader } from "./loader";

test("It renders correctly", () => {
  const { container } = render(<Loader />);
  expect(container).toMatchSnapshot();
});
