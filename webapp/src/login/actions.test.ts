import { OvermindMock, Config } from "overmind";
import { createTestOvermind } from "../overmind/test";
import { AuthenticationError } from "./effects";
import { ValidationErrors } from "../api/validation";

describe("checkUsername", () => {
  let checkUsernameEffect: jest.Mock;
  let overmind: OvermindMock<Config>;

  beforeEach(() => {
    checkUsernameEffect = jest.fn();

    overmind = createTestOvermind({
      login: {
        api: {
          checkUsername: checkUsernameEffect
        }
      }
    });
  });
  test("When the username is known", async () => {
    checkUsernameEffect.mockReturnValueOnce(Promise.resolve(true));
    await overmind.actions.login.checkUsername("testuser");
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
  });
  test("When the username is not known", async () => {
    checkUsernameEffect.mockReturnValueOnce(Promise.resolve(false));
    await overmind.actions.login.checkUsername("testuser");
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
  });
  test("When the server call fails", async () => {
    checkUsernameEffect.mockReturnValueOnce(
      Promise.reject(new Error("Network Error"))
    );
    await overmind.actions.login.checkUsername("testuser");
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
  });
});

describe("resetLogin", () => {
  test("Resetting from the initial state", () => {
    const overmind = createTestOvermind();
    overmind.actions.login.resetLogin();
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
  });
  test("Resetting from the registering state", () => {
    const overmind = createTestOvermind(
      {},
      {
        login: {
          mode: {
            current: "registering"
          }
        }
      }
    );

    overmind.actions.login.resetLogin();
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
  });
  test("Resetting from the authenticating state", () => {
    const overmind = createTestOvermind(
      {},
      {
        login: {
          mode: {
            current: "authenticating"
          }
        }
      }
    );
    overmind.actions.login.resetLogin();
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
  });
});

describe("cancelLogin", () => {
  test("Cancelling from the initial state", () => {
    const overmind = createTestOvermind();
    overmind.actions.login.cancelLogin();
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
  });
  test("Cancelling from the registering state", () => {
    const overmind = createTestOvermind(
      {},
      {
        login: {
          mode: {
            current: "registering"
          }
        }
      }
    );
    overmind.actions.login.cancelLogin();
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
  });
  test("Cancelling from the authenticating state", () => {
    const overmind = createTestOvermind(
      {},
      {
        login: {
          mode: {
            current: "authenticating"
          }
        }
      }
    );
    overmind.actions.login.cancelLogin();
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
  });
});

describe("authenticate", () => {
  let authenticateUserEffect: jest.Mock;
  let overmind: OvermindMock<Config>;

  beforeEach(() => {
    authenticateUserEffect = jest.fn();

    overmind = createTestOvermind(
      {
        login: {
          api: {
            authenticateUser: authenticateUserEffect
          }
        }
      },
      {
        login: {
          mode: {
            current: "authenticating"
          }
        }
      }
    );
  });
  test("Successfully", async () => {
    authenticateUserEffect.mockReturnValueOnce(Promise.resolve(undefined));
    const result = await overmind.actions.login.authenticate({
      username: "testuser",
      password: "Pa55word"
    });
    expect(result).toBeUndefined;
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
    expect(authenticateUserEffect).toBeCalledTimes(1);
    expect(authenticateUserEffect).toBeCalledWith("testuser", "Pa55word");
  });
  test("Invalid Password", async () => {
    authenticateUserEffect.mockReturnValueOnce(
      Promise.reject(new AuthenticationError())
    );
    const result = await overmind.actions.login.authenticate({
      username: "testuser",
      password: "Pa55word"
    });
    expect(result).toEqual(new AuthenticationError());
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
    expect(authenticateUserEffect).toBeCalledTimes(1);
    expect(authenticateUserEffect).toBeCalledWith("testuser", "Pa55word");
  });
  test("Network Error", async () => {
    authenticateUserEffect.mockReturnValueOnce(
      Promise.reject(new Error("Network Error"))
    );
    const result = await overmind.actions.login.authenticate({
      username: "testuser",
      password: "Pa55word"
    });
    expect(result).toEqual(undefined);
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
    expect(authenticateUserEffect).toBeCalledTimes(1);
    expect(authenticateUserEffect).toBeCalledWith("testuser", "Pa55word");
  });
});

describe("register", () => {
  let registerUserEffect: jest.Mock;
  let overmind: OvermindMock<Config>;

  beforeEach(() => {
    registerUserEffect = jest.fn();

    overmind = createTestOvermind(
      {
        login: {
          api: {
            registerUser: registerUserEffect
          }
        }
      },
      {
        login: {
          mode: {
            current: "registering"
          }
        }
      }
    );
  });
  test("Successfully", async () => {
    registerUserEffect.mockReturnValueOnce(Promise.resolve(undefined));
    const result = await overmind.actions.login.register({
      username: "testuser",
      email: "test@example.com",
      displayName: "Test User",
      password: "Pa55word"
    });
    expect(result).toBeUndefined;
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
    expect(registerUserEffect).toBeCalledTimes(1);
    expect(registerUserEffect).toBeCalledWith(
      "testuser",
      "test@example.com",
      "Test User",
      "Pa55word"
    );
  });
  test("Invalid Password", async () => {
    registerUserEffect.mockReturnValueOnce(
      Promise.reject(
        new ValidationErrors([
          {
            field: "email",
            type: "tag:universe,2020:users/validation-errors/email/duplicate"
          }
        ])
      )
    );
    const result = await overmind.actions.login.register({
      username: "testuser",
      email: "test@example.com",
      displayName: "Test User",
      password: "Pa55word"
    });
    expect(result).toEqual(
      new ValidationErrors([
        {
          field: "email",
          type: "tag:universe,2020:users/validation-errors/email/duplicate"
        }
      ])
    );
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
    expect(registerUserEffect).toBeCalledTimes(1);
    expect(registerUserEffect).toBeCalledWith(
      "testuser",
      "test@example.com",
      "Test User",
      "Pa55word"
    );
  });
  test("Network Error", async () => {
    registerUserEffect.mockReturnValueOnce(
      Promise.reject(new Error("Network Error"))
    );
    const result = await overmind.actions.login.register({
      username: "testuser",
      email: "test@example.com",
      displayName: "Test User",
      password: "Pa55word"
    });
    expect(result).toEqual(undefined);
    expect(overmind.mutations).toMatchSnapshot();
    expect(overmind.state).toMatchSnapshot();
    expect(registerUserEffect).toBeCalledTimes(1);
    expect(registerUserEffect).toBeCalledWith(
      "testuser",
      "test@example.com",
      "Test User",
      "Pa55word"
    );
  });
});
