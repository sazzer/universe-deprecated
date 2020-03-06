import { Url, PageName } from "./pageModel";
import { BasePage } from "./basePage";
import { WebElement } from "selenium-webdriver";
import { When } from "cucumber";

/**
 * Page Object representing the home page of the application
 */
@PageName("home")
@Url("/")
export class HomePage extends BasePage {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  async verifyCorrectPage() {
    return true;
  }
}

When("I go to the home page", async function() {
  const homePage = await this.browser.newPageModel(BasePage);
  const header = await homePage.headerBar();
  await header.goHome();
});
