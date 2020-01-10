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

  /**
   * Visit the page that is represnted by the given page model
   * @param  {Constructor}  page The page model to open
   */
  async visit(page) {
    if (page.URL) {
      const driver = await this._openBrowser();
      const urlBase = process.env.SERVICE_URL;
      const url = urlBase + page.URL;
      await driver.get(url);
    }
    return await this.buildPage(page);
  }

  /**
   * Build a page model as described by the given class
   * @param  {Constructor}  page The page model to open
   */
  async buildPage(page) {
    const driver = await this._openBrowser();
    return new page(driver);
  }
}

module.exports = { Browser };
