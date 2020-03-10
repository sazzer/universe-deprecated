import { By, WebElement } from "selenium-webdriver";
import { PageName, Url } from "../pageModel";

import { BasePage } from "../basePage";
import { When } from "cucumber";

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

  async openChangePassword() {
    const tab = await this.findElement(
      By.css('a[aria-controls="profile-tabs-password"]')
    );
    await tab.click();
  }
}

When("I load the user profile", async function() {
  const homePage = await this.browser.newPageModel(BasePage);
  const header = await homePage.headerBar();
  await header.openProfile();
});

When("I open the Change Password form", async function() {
  const profilePage = await this.browser.newPageModel(UserProfilePage);
  await profilePage.openChangePassword();
});
