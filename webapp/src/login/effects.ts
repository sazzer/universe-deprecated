import { request, ProblemResponse } from '../api';

async function checkUsername(username: string): Promise<boolean> {
  try {
    await request({
      url: '/usernames/{username}',
      urlParams: {
        username
      },
      method: 'GET',
    });

    return true;
  } catch (e) {
    if (e instanceof ProblemResponse && e.problem.type === 'tag:universe,2020:users/problems/unknown-user') {
      return false;
    } else {
      throw e;
    }
  }
}

export const api = {
  checkUsername,
}
