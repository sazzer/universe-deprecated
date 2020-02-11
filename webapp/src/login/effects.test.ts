import nock from "nock";
import { api } from "./effects";

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
