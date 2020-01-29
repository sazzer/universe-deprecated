import { Then } from 'cucumber';
import { HomePage } from '../page/homePage';
import { expect } from 'chai';

Then('I am not logged in', async function() {
  const homePage = await this.browser.newPageModel(HomePage);
  const header = await homePage.headerBar();
  const loggedIn = await header.isLoggedIn();
  expect(loggedIn, 'Currently logged in').to.be.false;
});
