import { WebElement, By } from "selenium-webdriver";
import { HeaderPageModel } from "./header";
import { BasePageModel } from "./pageModel";

/**
 * Base class for all full-page page models, giving common access to standard page features such as the header bar
 */
export class BasePage extends BasePageModel {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  /**
   * Get the page model for the page header
   * @return The page header
   */
  async headerBar() {
    const headerElement = await this.findElement(By.css("nav.navbar"));
    return new HeaderPageModel(headerElement);
  }

  async verifyCorrectPage() {
    return true;
  }
}
