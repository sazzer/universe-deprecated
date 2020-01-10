const { BasePage } = require('./basepage');
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
   * Enter a username to log in as
   * @param  {string}  username The username to enter
   */
  async login(username) {
    const usernameField = this._element.findElement(By.css('#login_username'));
    await usernameField.clear();
    await usernameField.sendKeys(username);

    const submit = this._element.findElement(By.css('form button.btn-primary[type="submit"]'));
    await submit.click();
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
    const fields = {
      Username: 'input[name="username"]',
      'Email Address': 'input[name="email"]',
      'Display Name': 'input[name="name"]',
      Password: 'input[name="password"]',
      'Re-enter Password': 'input[name="password2"]',
    }

    const data = {};
    for (const field of Object.keys(fields)) {
      const selector = fields[field];
      const element = await this._element.findElement(By.css(selector));
      const value = await element.getAttribute('value');
      data[field] = value;
    }

    return data;
  }
}
module.exports = { StartLoginPage, RegisterPage };
