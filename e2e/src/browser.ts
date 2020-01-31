import { Builder, WebDriver, By } from 'selenium-webdriver';
import { AfterAll } from 'cucumber';
import debug from 'debug';
import { PageModelConstructor, PageModel } from './page/pageModel';

const LOG = debug('universe:e2e:browser');

/** The raw web driver */
let _driver: WebDriver | undefined;

/**
 * Get the Web Driver, creating a new one if needed
 * @return The Web Driver
 */
async function getWebDriver() {
  if (_driver === undefined) {
    LOG('Creating new Web Driver');
    _driver = await new Builder()
      .forBrowser('chrome')
      .build();
    _driver.manage().setTimeouts({ implicit: 20000, pageLoad: 10000 });
  }

  return _driver;
}


/**
 * Destroy the browser so that we can start fresh next time
 */
async function destroyWebDriver() {
  if (_driver !== undefined) {
    LOG('Destroying Web Driver');
    await _driver.close();
    _driver = undefined;
  }
}

AfterAll(async () => {
  await destroyWebDriver();
})

/**
 * Wrapper around the web browser, via Selenium
 */
export class Browser {
  /**
   * Reset the Browser to a known-good state
   */
  async reset() {
    await getWebDriver();
  }

  /**
   * Destroy the browser so that we can start fresh next time
   */
  async destroy() {
    destroyWebDriver();
  }

  /**
   * Take a screenshot of the current state of the browser
   * @return The screenshot
   */
  async screenshot() {
    LOG('Taking screenshot');
    const driver = await getWebDriver();
    return await driver.takeScreenshot();
  }

  /**
   * Construct a new page model for the given constructor
   * @return The constructor of the Page Model class
   */
  async newPageModel<T extends PageModel>(constructor: PageModelConstructor<T>): Promise<T> {
    const driver = await getWebDriver();
    const baseElement = await driver.wait(() => driver.findElement(By.tagName("body")));
    return new constructor(baseElement);
  }

  /**
   * Direct the browser to visit the given URL
   * @param  url The URL to visit
   */
  async visitPage(url: string) {
    const urlBase = process.env.WEBAPP_URL;
    const realUrl = urlBase + url;
    LOG('Visiting page: %s', realUrl);

    const driver = await getWebDriver();
    await driver.get(realUrl);
  }
}
