const { When, Then } = require('cucumber');
const { BasePage } = require('./pages/basepage');
const { StartLoginPage, RegisterPage } = require('./pages/login');
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

Then('I am displayed the Start Login form', async function() {
  await this.browser.buildPage(StartLoginPage);
});

Then('the Start Login form has details:', async function(data) {
  const page = await this.browser.buildPage(StartLoginPage);
  const form = await page.getForm();
  const values = await form.getValues();

  const expected = data.rowsHash();
  expect(values).to.include(expected);
});

Then('the Start Login form has no errors', async function() {
  const page = await this.browser.buildPage(StartLoginPage);
  const form = await page.getForm();
  const values = await form.getErrors();

  expect(values).to.be.empty;
});

Then('the Start Login form has errors:', async function(data) {
  const page = await this.browser.buildPage(StartLoginPage);
  const form = await page.getForm();
  const values = await form.getErrors();

  const expected = data.rowsHash();
  expect(values).to.include(expected);
});

Then('I am displayed the Register User form', async function() {
  await this.browser.buildPage(RegisterPage);
});

Then('the Register User form has details:', async function(data) {
  const page = await this.browser.buildPage(RegisterPage);
  const form = await page.getForm();
  const values = await form.getValues();

  const expected = data.rowsHash();
  expect(values).to.include(expected);
});
