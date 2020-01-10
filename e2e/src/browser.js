const { Builder, By, Key, until } = require('selenium-webdriver');

/**
 * The wrapper around the Browser to use
 */
class Browser {
  /**
   * Construct the browser to use, opening up a Selenium WebDriver to make use of
   */
  constructor() {
    console.log('Constructing browser');
  }

  /**
   * Actually ensure that we have a browser open, and open one if not
   */
  async _openBrowser() {
    if (this._driver === undefined) {
      this._driver = await new Builder()
        .forBrowser('chrome')
        .build();
    }
    return this._driver;
  }

  /**
   * Reset the browser by clearing out any per-session state
   */
  async reset() {
    console.log('Resetting browser');
    const driver = await this._openBrowser();
    await driver.get('http://www.google.com');
  }

  /**
   * Close the browser down
   */
  async close() {
    console.log('Closing browser');
    const driver = await this._openBrowser();
    await driver.quit();
  }

  /**
   * Take a screenshot of the current browser view
   */
  async screenshot() {
    const driver = await this._openBrowser();
    return await driver.takeScreenshot();
  }
}

module.exports = { Browser };
