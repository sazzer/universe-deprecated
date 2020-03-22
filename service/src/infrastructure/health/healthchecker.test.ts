import { Healthchecker } from "./healthchecker";

describe("Healthchecker", () => {
  test("With no checks", async () => {
    const testSubject = new Healthchecker({});
    const result = await testSubject.check();

    expect(result).toEqual({
      status: "PASS",
      components: {}
    });
  });

  test("With one passing check", async () => {
    const testSubject = new Healthchecker({
      passing: async () => {
        return {
          status: "PASS",
          message: "Success"
        };
      }
    });
    const result = await testSubject.check();

    expect(result).toEqual({
      status: "PASS",
      components: {
        passing: {
          status: "PASS",
          message: "Success"
        }
      }
    });
  });

  test("With one failing check", async () => {
    const testSubject = new Healthchecker({
      failing: async () => {
        return {
          status: "FAIL",
          message: "Error"
        };
      }
    });
    const result = await testSubject.check();

    expect(result).toEqual({
      status: "FAIL",
      components: {
        failing: {
          status: "FAIL",
          message: "Error"
        }
      }
    });
  });

  test("With mixed checks", async () => {
    const testSubject = new Healthchecker({
      passing: async () => {
        return {
          status: "PASS",
          message: "Success"
        };
      },
      failing: async () => {
        return {
          status: "FAIL",
          message: "Error"
        };
      }
    });
    const result = await testSubject.check();

    expect(result).toEqual({
      status: "FAIL",
      components: {
        passing: {
          status: "PASS",
          message: "Success"
        },
        failing: {
          status: "FAIL",
          message: "Error"
        }
      }
    });
  });
});
