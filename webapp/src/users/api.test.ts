import * as api from "./api";

import nock from "nock";

const URL_BASE = "http://api.test.example.com";

beforeEach(() => {
  process.env.REACT_APP_SERVICE_URL = URL_BASE;
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
