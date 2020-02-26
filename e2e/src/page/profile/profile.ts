import { Url, PageName } from "../pageModel";
import { BasePage } from "../basePage";
import { WebElement, By } from "selenium-webdriver";

/**
 * Page Object representing the profile page of the application
 */
@PageName("User Profile")
@Url("/profile")
export class UserProfilePage extends BasePage {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  async verifyCorrectPage() {
    const form = await this.findElement(By.css('div[data-test="profilePage"]'));
    return await form.isDisplayed();
  }
}
