import { BasePage } from "../basePage";
import { FormPage, FormModelBuilder } from "../form";
import { WebElement, By } from "selenium-webdriver";

/**
 * Page Object representing the User Profile portion of the profile page
 */
@FormPage("User Profile")
export class UserProfilePage extends BasePage {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  async verifyCorrectPage() {
    const form = await this.findElement(
      By.css('form[data-test="userProfileForm"]')
    );
    return await form.isDisplayed();
  }

  async getForm() {
    const form = await this.findElement(
      By.css('form[data-test="userProfileForm"]')
    );

    return new FormModelBuilder(form)
      .withInput("Username", By.css('div.form-group[data-test="username"]'))
      .withInput("Email Address", By.css('div.form-group[data-test="email"]'))
      .withInput(
        "Display Name",
        By.css('div.form-group[data-test="displayName"]')
      )
      .build();
  }
}
