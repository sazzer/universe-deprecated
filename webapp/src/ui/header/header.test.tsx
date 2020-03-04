import { HeaderBar } from "./index";
import React from "react";
import { MemoryRouter as Router } from "react-router-dom";
import { render } from "@testing-library/react";

describe("When not logged in", () => {
  test("It renders correctly", () => {
    const { container } = render(
      <Router>
        <HeaderBar />
      </Router>
    );
    expect(container).toMatchSnapshot();
  });
});
