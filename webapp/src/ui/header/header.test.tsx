import React from "react";
import { render } from "@testing-library/react";
import { BrowserRouter as Router } from "react-router-dom";
import { Header } from "./header";
import { createTestOvermind } from "../../overmind/test";
import { Provider } from "overmind-react";

describe("Rendering the header bar", () => {
  test("When not logged in", () => {
    const overmind = createTestOvermind();
    const { container } = render(
      <Router>
        <Provider value={overmind}>
          <Header />
        </Provider>
      </Router>
    );
    expect(container).toMatchSnapshot();
  });
});
