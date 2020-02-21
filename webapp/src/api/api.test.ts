import { request, ProblemResponse, Response } from ".";
import nock from "nock";

const URL_BASE = "http://api.test.example.com";

describe("Making API calls", () => {
  beforeEach(() => {
    process.env.REACT_APP_SERVICE_URL = URL_BASE;
  });

  describe("Success responses", () => {
    it("Returns the correct response", async () => {
      nock(URL_BASE)
        .defaultReplyHeaders({ "access-control-allow-origin": "*" })
        .get("/a/b/c")
        .reply(200, {
          hello: "world"
        });

      const response = await request({
        url: "/a/b/c"
      });

      expect(response.status).toBe(200);
      expect(response.data).toEqual({
        hello: "world"
      });
    });

    it("Supports URI Templates in the path", async () => {
      nock(URL_BASE)
        .defaultReplyHeaders({ "access-control-allow-origin": "*" })
        .get("/a/42")
        .reply(200, {
          hello: "world"
        });

      const response = await request({
        url: "/a/{answer}",
        urlParams: {
          answer: 42
        }
      });

      expect(response.status).toBe(200);
      expect(response.data).toEqual({
        hello: "world"
      });
    });

    it("Supports URI Templates in the querystring", async () => {
      nock(URL_BASE)
        .defaultReplyHeaders({ "access-control-allow-origin": "*" })
        .get("/a?answer=42")
        .reply(200, {
          hello: "world"
        });

      const response = await request({
        url: "/a{?answer}",
        urlParams: {
          answer: 42
        }
      });

      expect(response.status).toBe(200);
      expect(response.data).toEqual({
        hello: "world"
      });
    });

    it("Supports missing URI Templates in the querystring", async () => {
      nock(URL_BASE)
        .defaultReplyHeaders({ "access-control-allow-origin": "*" })
        .get("/a")
        .reply(200, {
          hello: "world"
        });

      const response = await request({
        url: "/a{?answer}",
        urlParams: {}
      });

      expect(response.status).toBe(200);
      expect(response.data).toEqual({
        hello: "world"
      });
    });

    it("Allows a POST call to work", async () => {
      nock(URL_BASE)
        .defaultReplyHeaders({ "access-control-allow-origin": "*" })
        .post("/a/b/c", "username=pgte&password=123456")
        .reply(201, {
          hello: "world"
        });

      const response = await request({
        url: "/a/b/c",
        method: "POST",
        data: "username=pgte&password=123456"
      });

      expect(response.status).toBe(201);
      expect(response.data).toEqual({
        hello: "world"
      });
    });
  });

  describe("Expected problem responses", () => {
    it("Returns the correct response", async () => {
      nock(URL_BASE)
        .defaultReplyHeaders({ "access-control-allow-origin": "*" })
        .get("/a/b/c")
        .reply(
          404,
          {
            type: "tag:universe,2020:not-found",
            title: "Not Found",
            status: 404
          },
          {
            "content-type": "application/problem+json"
          }
        );

      try {
        await request({
          url: "/a/b/c"
        });
        fail("Should have thrown");
      } catch (e) {
        expect(e).toBeInstanceOf(ProblemResponse);
        const problemResponse = e as ProblemResponse<any>;
        expect(problemResponse.status).toBe(404);
        expect(problemResponse.problem).toEqual({
          type: "tag:universe,2020:not-found",
          title: "Not Found",
          status: 404
        });
      }
    });

    it("Allows extra fields", async () => {
      nock(URL_BASE)
        .defaultReplyHeaders({ "access-control-allow-origin": "*" })
        .get("/a/b/c")
        .reply(
          400,
          {
            type: "tag:universe,2020:invalid-data",
            title: "Invalid Data",
            status: 400,
            validation: "You can't do that"
          },
          {
            "content-type": "application/problem+json"
          }
        );

      try {
        await request({
          url: "/a/b/c"
        });
        fail("Should have thrown");
      } catch (e) {
        expect(e).toBeInstanceOf(ProblemResponse);
        const problemResponse = e as ProblemResponse<any>;
        expect(problemResponse.status).toBe(400);
        expect(problemResponse.problem).toEqual({
          type: "tag:universe,2020:invalid-data",
          title: "Invalid Data",
          status: 400,
          validation: "You can't do that"
        });
      }
    });
  });

  describe("Unexpected problem responses", () => {
    it("HTTP Response", async () => {
      nock(URL_BASE)
        .defaultReplyHeaders({ "access-control-allow-origin": "*" })
        .get("/a/b/c")
        .reply(500, "Broken Service", {
          "content-type": "text/plain"
        });

      try {
        await request({
          url: "/a/b/c"
        });
        fail("Should have thrown");
      } catch (e) {
        expect(e).toBeInstanceOf(Error);
        expect(e.response).toBeDefined();

        const problemResponse = e.response as Response<any>;

        expect(problemResponse.status).toBe(500);
        expect(problemResponse.data).toEqual("Broken Service");
      }
    });

    xit("Socket Timeout", async () => {
      nock(URL_BASE)
        .defaultReplyHeaders({ "access-control-allow-origin": "*" })
        .get("/a/b/c")
        .socketDelay(30000) // API Timeout is 20000
        .reply(500, "Broken Service", {
          "content-type": "text/plain"
        });

      try {
        await request({
          url: "/a/b/c"
        });
        fail("Should have thrown");
      } catch (e) {
        expect(e).toBeInstanceOf(Error);
        expect(e.response).toBeUndefined();
        expect(e.code).toEqual("ECONNABORTED");
      }
    });
  });
});
