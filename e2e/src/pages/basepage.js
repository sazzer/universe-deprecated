const { HeaderBar } = require('./header');
const { By } = require('selenium-webdriver');
/**
 * Page Model that represents the base constructs for the page
 */
class BasePage {
  constructor(element) {
    this._element = element;
  }

  /**
   * Get the header bar of the page
   */
  async getHeader() {
    const element = await this._element.findElement(By.css('nav.navbar'));
    return new HeaderBar(element);
  }
}

module.exports = { BasePage };
