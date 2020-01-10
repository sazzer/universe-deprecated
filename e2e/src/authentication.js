const { Then } = require('cucumber');
const { BasePage } = require('./pages/basepage');
const { expect } = require('chai');

Then('I am not logged in', async function() {
  const page = await this.browser.buildPage(BasePage);
  const header = await page.getHeader();
  const authenticated = await header.isLoggedIn();
  expect(authenticated).to.be.false;
});
