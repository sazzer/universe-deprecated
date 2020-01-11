const { When, Then } = require('cucumber');
const { BasePage } = require('./pages/basepage');
const { StartLoginPage, RegisterPage } = require('./pages/login');
const { buildPageSteps, buildFormSteps } = require('./stepHelpers');
const { expect } = require('chai');

Then('I am not logged in', async function() {
  const page = await this.browser.buildPage(BasePage);
  const header = await page.getHeader();
  const authenticated = await header.isLoggedIn();
  expect(authenticated).to.be.false;
});

When('I start logging in as {string}', async function(username) {
  const currentPage = await this.browser.buildPage(BasePage);
  const header = await currentPage.getHeader();
  await header.login();

  const loginPage = await this.browser.buildPage(StartLoginPage);
  await loginPage.login(username);
});

buildPageSteps('Start Login', StartLoginPage);
buildFormSteps('Start Login', StartLoginPage);

buildPageSteps('Register User', RegisterPage);
buildFormSteps('Register User', RegisterPage);
