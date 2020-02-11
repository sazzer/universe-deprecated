import { createOvermindMock, OvermindMock, Config } from "overmind";
import { config } from "../overmind";
import { cloneDeep, merge } from "lodash";
import { createTestOvermind } from "../overmind/test";

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
