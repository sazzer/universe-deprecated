const { Before, AfterAll, After, setWorldConstructor, setDefaultTimeout } = require('cucumber');
const { Browser } = require('./browser');

/** The long term browser instance to use */
const theBrowser = new Browser();

/**
 * The Cucumber World. A new instance of this is created for each Scenario
 */
class World {
  /**
   * Create a new world for the scenario
   */
  constructor({ attach, parameters }) {
    this.browser = theBrowser;
    this.attach = attach;
    this.parameters = parameters;
  }
}

Before(async function() {
  await this.browser.reset();
});

After(async function() {
  const screenshot = await this.browser.screenshot();
  this.attach(screenshot, 'image/png');
});

AfterAll(async function() {
  await theBrowser.close();
});

setWorldConstructor(World);
setDefaultTimeout(60 * 1000);
