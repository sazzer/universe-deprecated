import * as api from "./api";

import { ValidationErrors, setAccessToken } from "../api";

import nock from "nock";

const URL_BASE = "http://api.test.example.com";

beforeEach(() => {
  process.env.REACT_APP_SERVICE_URL = URL_BASE;
  setAccessToken(undefined);
});

describe("checkUsername", () => {
  test("Known username", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .get("/usernames/testuser")
      .reply(200, {
        hello: "world"
      });

    const result = await api.checkUsername("testuser");
    expect(result).toBe(true);
  });
  test("Unknown username", async () => {
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
  test("Network error", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .get("/usernames/testuser")
      .reply(500, "Broken Service", {
        "content-type": "text/plain"
      });

    try {
      await api.checkUsername("testuser");
      fail("Expected an exception");
    } catch (e) {
      expect(e.toString()).toBe("Error: Request failed with status code 500");
    }
  });
});

describe("getUserById", () => {
  test("Successfully", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .get("/users/57c33107-b43e-4b53-a967-3ff89ccaaf16")
      .reply(200, {
        id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
        email: "testuser@example.com",
        username: "testuser",
        displayName: "Test User"
      });

    const user = await api.getUserById("57c33107-b43e-4b53-a967-3ff89ccaaf16");

    expect(user).toEqual({
      id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    });
  });

  test("Unknown User", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .get("/users/57c33107-b43e-4b53-a967-3ff89ccaaf16")
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

    try {
      await api.getUserById("57c33107-b43e-4b53-a967-3ff89ccaaf16");
      fail("Expected an exception");
    } catch (e) {
      expect(e.toString()).toBe("Error: Not Found");
    }
  });
});

describe("authenticate", () => {
  test("Successfully", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/login", {
        username: "testuser",
        password: "Pa55word"
      })
      .reply(200, {
        id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
        email: "testuser@example.com",
        username: "testuser",
        displayName: "Test User",
        accessToken: {
          token: "someAccessToken",
          expires: "2020-03-05T23:59:59Z"
        }
      });

    const user = await api.authenticate("testuser", "Pa55word");

    expect(user).toEqual({
      id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    });
  });
  test("Wrong Password", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/login", {
        username: "testuser",
        password: "Pa55word"
      })
      .reply(
        400,
        {
          type: "tag:universe,2020:users/problems/login_failure",
          title: "Login Failure",
          status: 400
        },
        {
          "content-type": "application/problem+json"
        }
      );

    try {
      await api.authenticate("testuser", "Pa55word");
      fail("Expected an exception");
    } catch (e) {
      expect(e).toBeInstanceOf(api.LoginFailure);
    }
  });
  test("Unexpected error", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/login", {
        username: "testuser",
        password: "Pa55word"
      })
      .reply(500, "Broken Service", {
        "content-type": "text/plain"
      });

    try {
      await api.authenticate("testuser", "Pa55word");
      fail("Expected an exception");
    } catch (e) {
      expect(e.toString()).toBe("Error: Request failed with status code 500");
    }
  });
});

describe("register", () => {
  test("Successfully", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/users", {
        username: "testuser",
        email: "testuser@example.com",
        displayName: "Test User",
        password: "Pa55word"
      })
      .reply(200, {
        id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
        email: "testuser@example.com",
        username: "testuser",
        displayName: "Test User",
        accessToken: {
          token: "someAccessToken",
          expires: "2020-03-05T23:59:59Z"
        }
      });

    const user = await api.register(
      "testuser",
      "testuser@example.com",
      "Test User",
      "Pa55word"
    );

    expect(user).toEqual({
      id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    });
  });
  test("Duplicate Email Address", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/users", {
        username: "testuser",
        email: "testuser@example.com",
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
      await api.register(
        "testuser",
        "testuser@example.com",
        "Test User",
        "Pa55word"
      );
      fail("Expected an exception");
    } catch (e) {
      expect(e).toBeInstanceOf(ValidationErrors);
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
  test("Unexpected error", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/users", {
        username: "testuser",
        email: "testuser@example.com",
        displayName: "Test User",
        password: "Pa55word"
      })
      .reply(500, "Broken Service", {
        "content-type": "text/plain"
      });

    try {
      await api.register(
        "testuser",
        "testuser@example.com",
        "Test User",
        "Pa55word"
      );
      fail("Expected an exception");
    } catch (e) {
      expect(e.toString()).toBe("Error: Request failed with status code 500");
    }
  });
});

describe("updateUserProfile", () => {
  const userId = "57c33107-b43e-4b53-a967-3ff89ccaaf16";

  beforeEach(() => {
    nock(URL_BASE)
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "access-control-allow-method": "*"
      })
      .options(`/users/${userId}`)
      .reply(204);
  });
  test("Successfully", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({
        "access-control-allow-origin": "*"
      })
      .patch(`/users/${userId}`, {
        email: "testuser@example.com",
        displayName: "Test User"
      })
      .reply(200, {
        id: userId,
        email: "testuser@example.com",
        username: "testuser",
        displayName: "Test User"
      });

    const user = await api.updateUserProfile(
      userId,
      "testuser@example.com",
      "Test User"
    );

    expect(user).toEqual({
      id: userId,
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    });
  });
  test("Duplicate Email Address", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .patch(`/users/${userId}`, {
        email: "testuser@example.com",
        displayName: "Test User"
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
      await api.updateUserProfile(userId, "testuser@example.com", "Test User");
      fail("Expected an exception");
    } catch (e) {
      expect(e).toBeInstanceOf(ValidationErrors);
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
  test("Unexpected error", async () => {
    nock(URL_BASE)
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .patch(`/users/${userId}`, {
        email: "testuser@example.com",
        displayName: "Test User"
      })
      .reply(500, "Broken Service", {
        "content-type": "text/plain"
      });

    try {
      await api.updateUserProfile(userId, "testuser@example.com", "Test User");
      fail("Expected an exception");
    } catch (e) {
      expect(e.toString()).toBe("Error: Request failed with status code 500");
    }
  });
});
