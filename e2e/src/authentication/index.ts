import { Then, When, TableDefinition } from 'cucumber';
import { HomePage } from '../page/homePage';
import { StartLoginPage } from '../page/login/start';
import { RegisterUserPage } from '../page/login/register';
import { expect } from 'chai';
import { processObject } from '../table';

Then('I am not logged in', async function() {
  const homePage = await this.browser.newPageModel(HomePage);
  const header = await homePage.headerBar();
  const loggedIn = await header.isLoggedIn();
  expect(loggedIn, 'Currently logged in').to.be.false;
});

When('I start logging in as {string}', async function(username: string) {
  const homePage = await this.browser.newPageModel(HomePage);
  const header = await homePage.headerBar();
  await header.startLogin();

  const startLoginPage = await this.browser.newPageModel(StartLoginPage);
  const startLoginForm = await startLoginPage.getForm();
  await startLoginForm.setField('Username', username);
  await startLoginForm.submit();
});

When('I register with details:', async function(details: TableDefinition) {
  const registerPage = await this.browser.newPageModel(RegisterUserPage);
  const registerForm = await registerPage.getForm();
  await registerForm.setAllValues(processObject(details.rowsHash()));
  await registerForm.submit();
});
