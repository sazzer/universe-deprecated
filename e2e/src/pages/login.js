const { By } = require('selenium-webdriver');
const { BasePage } = require('./basepage');
const { FormPage } = require('./formpage');
const { buildPageSteps, buildFormSteps } = require('../stepHelpers');

/**
 * Page Model that represents the page for starting to log in
 * @extends BasePage
 */
class StartLoginPage extends BasePage {
  constructor(element) {
    super(element);
  }

  /**
   * Get the contents of the form as currently displayed
   */
  async getForm() {
    const form = await this._element.findElement(By.css('form[action="/login"]'));
    const fields = {
      Username: 'div[data-test="username"]',
    }

    return new FormPage(form, fields);
  }
}
StartLoginPage.URL = '/login';

/**
 * Page model for the page to register a new user
 * @extends BasePage
 */
class RegisterPage extends BasePage {
  constructor(element) {
    super(element);
  }

  /**
   * Check if this page is the one that is currently displayed
   */
  async isDisplayed() {
    const form = await this._element.findElement(By.css('form[action="/login/register"]'));
    return await form.isDisplayed();
  }

  /**
   * Get the contents of the form as currently displayed
   */
  async getForm() {
    const form = await this._element.findElement(By.css('form[action="/login/register"]'));
    const fields = {
      Username: 'div[data-test="username"]',
      'Email Address': 'div[data-test="email"]',
      'Display Name': 'div[data-test="name"]',
      Password: 'div[data-test="password"]',
      'Re-enter Password': 'div[data-test="password2"]',
    }

    return new FormPage(form, fields);
  }
}

/**
 * Page model for the page to log in as a known user
 * @extends BasePage
 */
class LoginPage extends BasePage {
  constructor(element) {
    super(element);
  }

  /**
   * Check if this page is the one that is currently displayed
   */
  async isDisplayed() {
    const form = await this._element.findElement(By.css('form[action="/login/login"]'));
    return await form.isDisplayed();
  }

  /**
   * Get the contents of the form as currently displayed
   */
  async getForm() {
    const form = await this._element.findElement(By.css('form[action="/login/login"]'));
    const fields = {
      Username: 'div[data-test="username"]',
      Password: 'div[data-test="password"]',
    }

    return new FormPage(form, fields);
  }
}

buildPageSteps('Start Login', StartLoginPage);
buildFormSteps('Start Login', StartLoginPage);

buildPageSteps('Register User', RegisterPage);
buildFormSteps('Register User', RegisterPage);

buildPageSteps('Login User', LoginPage);
buildFormSteps('Login User', LoginPage);

module.exports = { StartLoginPage, RegisterPage, LoginPage };
