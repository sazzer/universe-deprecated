import nock from "nock";
import { api, AuthenticationError } from "./effects";
import { ValidationErrors } from "../../api/validation";

const URL_BASE = "http://api.test.example.com";

describe("checkUsername", () => {
  beforeEach(() => {
    process.env.REACT_APP_SERVICE_URL = URL_BASE;
  });

  test("When the username is known", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .get("/usernames/testuser")
      .reply(200, {});

    const result = await api.checkUsername("testuser");
    expect(result).toBe(true);
  });

  test("When the username is unknown", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .get("/usernames/testuser")
      .reply(
        404,
        {
          type: "tag:universe,2020:users/problems/unknown-user",
          title: "Not Found",
          status: 404
        },
        {
          "content-type": "application/problem+json"
        }
      );

    const result = await api.checkUsername("testuser");
    expect(result).toBe(false);
  });

  test("When the HTTP call fails", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .get("/usernames/testuser")
      .reply(500, "Broken Service", {
        "content-type": "text/plain"
      });

    try {
      await api.checkUsername("testuser");
      fail("Exception expected");
    } catch (e) {
      expect(e.toString()).toEqual(
        "Error: Request failed with status code 500"
      );
    }
  });
});

describe("authenticateUser", () => {
  beforeEach(() => {
    process.env.REACT_APP_SERVICE_URL = URL_BASE;
  });

  test("Successfully", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/login", { username: "testuser", password: "Pa55word" })
      .reply(200, {
        id: "some-user-id",
        username: "testuser",
        email: "test@example.com",
        displayName: "Test User",
        accessToken: {
          token: "some.token.id",
          expiry: "2021-02-19T18:43:41.310201Z"
        }
      });

    const result = await api.authenticateUser("testuser", "Pa55word");
    expect(result).toStrictEqual({
      id: "some-user-id",
      username: "testuser",
      email: "test@example.com",
      displayName: "Test User",
      accessToken: {
        token: "some.token.id",
        expiry: "2021-02-19T18:43:41.310201Z"
      }
    });
  });

  test("Unsuccessfully", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/login", { username: "testuser", password: "Pa55word" })
      .reply(
        400,
        {
          type: "tag:universe,2020:users/problems/login_failure",
          title: "Invalid Username or Password",
          status: 400
        },
        {
          "content-type": "application/problem+json"
        }
      );

    try {
      await api.authenticateUser("testuser", "Pa55word");
      fail("Expected an AuthenticationError");
    } catch (e) {
      expect(e).toEqual(new AuthenticationError());
    }
  });

  test("When the HTTP call fails", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/login", { username: "testuser", password: "Pa55word" })
      .reply(500, "Broken Service", {
        "content-type": "text/plain"
      });

    try {
      await api.authenticateUser("testuser", "Pa55word");
      fail("Exception expected");
    } catch (e) {
      expect(e.toString()).toEqual(
        "Error: Request failed with status code 500"
      );
    }
  });
});

describe("registerUser", () => {
  beforeEach(() => {
    process.env.REACT_APP_SERVICE_URL = URL_BASE;
  });

  test("Successfully", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/users", {
        username: "testuser",
        email: "test@example.com",
        displayName: "Test User",
        password: "Pa55word"
      })
      .reply(200, {
        id: "some-user-id",
        username: "testuser",
        email: "test@example.com",
        displayName: "Test User",
        accessToken: {
          token: "some.token.id",
          expiry: "2021-02-19T18:43:41.310201Z"
        }
      });

    const result = await api.registerUser(
      "testuser",
      "test@example.com",
      "Test User",
      "Pa55word"
    );
    expect(result).toStrictEqual({
      id: "some-user-id",
      username: "testuser",
      email: "test@example.com",
      displayName: "Test User",
      accessToken: {
        token: "some.token.id",
        expiry: "2021-02-19T18:43:41.310201Z"
      }
    });
  });

  test("Unsuccessfully", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/users", {
        username: "testuser",
        email: "test@example.com",
        displayName: "Test User",
        password: "Pa55word"
      })
      .reply(
        422,
        {
          type: "tag:universe,2020:problems/validation-error",
          title: "The input had validation errors",
          status: 422,
          errors: [
            {
              field: "email",
              type: "tag:universe,2020:users/validation-errors/email/duplicate"
            }
          ]
        },
        {
          "content-type": "application/problem+json"
        }
      );

    try {
      await api.registerUser(
        "testuser",
        "test@example.com",
        "Test User",
        "Pa55word"
      );
      fail("Expected ValidationErrors");
    } catch (e) {
      expect(e).toEqual(
        new ValidationErrors([
          {
            field: "email",
            type: "tag:universe,2020:users/validation-errors/email/duplicate"
          }
        ])
      );
    }
  });

  test("When the HTTP call fails", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/users", {
        username: "testuser",
        email: "test@example.com",
        displayName: "Test User",
        password: "Pa55word"
      })
      .reply(500, "Broken Service", {
        "content-type": "text/plain"
      });

    try {
      await api.registerUser(
        "testuser",
        "test@example.com",
        "Test User",
        "Pa55word"
      );
      fail("Exception expected");
    } catch (e) {
      expect(e.toString()).toEqual(
        "Error: Request failed with status code 500"
      );
    }
  });
});
