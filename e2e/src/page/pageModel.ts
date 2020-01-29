import debug from 'debug';
import { When } from 'cucumber';
import { WebElement, By } from 'selenium-webdriver';
import { expect } from 'chai';

const LOG = debug('universe:e2e:basePage');

export interface PageModel {
  url?: string;
  verifyCorrectPage(): Promise<boolean>;
}

export interface PageModelConstructor<T extends PageModel> {
  new(driver: WebElement): T
}

export function Url(url: string) {
  return function <T extends PageModel>(constructor: PageModelConstructor<T>) {
    constructor.prototype.url = url;
  }
}

export function PageName(name: string) {
  return function <T extends PageModel>(constructor: PageModelConstructor<T>) {
    LOG('Building page steps with name "%s" for page model: %o', name, constructor);

    When(`I visit the ${name}`, async function() {
      LOG('Visiting page: %o', constructor);

      const page = await this.browser.newPageModel(constructor);

      if (page.url) {
        this.browser.visitPage(page.url);
      }

      const correctPage = await page.verifyCorrectPage();
      expect(correctPage, `Visiting page [${name}]`).to.be.true;
    });
  }
}


/**
 * Base class for all page models to give access to common functionality
 */
export class BasePageModel {
  /** The web element at the base of this page model */
  private _baseElement: WebElement;

  constructor(baseElement: WebElement) {
    this._baseElement = baseElement;
  }

  /**
   * Find an element with the given selector
   * @param  by The selector
   * @return    The element that was found
   */
  protected async findElement(by: By) {
    return this._baseElement.findElement(by);
  }
}
