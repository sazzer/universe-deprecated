const { By } = require('selenium-webdriver');

/**
 * Page Model to represent the header bar
 */
class HeaderBar {
  constructor(element) {
    this._element = element;
  }

  /**
   * Determine if we're logged in or not.
   * This is done by the presence of the "Login / Register" link.
   */
  async isLoggedIn() {
    const loginLink = await this._element.findElement(By.css('a.nav-link[href="/login"]'));
    const loginLinkPresent = await loginLink.isDisplayed();
    return !loginLinkPresent;
  }

  /**
   * Attempt to start authentication by pressing the "Login / Register" link
   */
  async login() {
    const loginLink = await this._element.findElement(By.css('a.nav-link[href="/login"]'));
    await loginLink.click();
  }
}

module.exports = { HeaderBar };
