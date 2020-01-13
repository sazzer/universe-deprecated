const { When, Then } = require('cucumber');
const { BasePage } = require('./pages/basepage');
const { StartLoginPage, RegisterPage } = require('./pages/login');
const { buildPageSteps, buildFormSteps } = require('./stepHelpers');
const { expect } = require('chai');
const { parseValue } = require('./dataTable');

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
  const loginForm = await loginPage.getForm();
  await loginForm.populate({
    Username: username,
  });
  await loginForm.submit();
});

When('I register with details:', async function(data) {
  const newValues = {};
  const input = data.rowsHash();
  Object.keys(input).forEach(key => {
    newValues[key] = parseValue(input[key]);
  });

  const registrationPage = await this.browser.buildPage(RegisterPage);
  const registrationForm = await registrationPage.getForm();
  await registrationForm.populate(newValues);
  await registrationForm.submit();
});

buildPageSteps('Start Login', StartLoginPage);
buildFormSteps('Start Login', StartLoginPage);

buildPageSteps('Register User', RegisterPage);
buildFormSteps('Register User', RegisterPage);
