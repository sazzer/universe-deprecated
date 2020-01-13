const { BasePage } = require('./basepage');
const { FormPage } = require('./formpage');
const { By } = require('selenium-webdriver');

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
module.exports = { StartLoginPage, RegisterPage };
