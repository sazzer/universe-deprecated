import { By, WebElement } from "selenium-webdriver";
import { FormModelBuilder, FormPage } from "../form";

import { BasePage } from "../basePage";

/**
 * Page Object representing the Change Password portion of the profile page
 */
@FormPage("Change Password")
export class ChangePasswordPage extends BasePage {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  async verifyCorrectPage() {
    const form = await this.findElement(
      By.css('form[data-test="changePasswordForm"]')
    );
    return await form.isDisplayed();
  }

  async getForm() {
    const form = await this.findElement(
      By.css('form[data-test="changePasswordForm"]')
    );

    return new FormModelBuilder(form)
      .withInput("New Password", By.css('div.form-group[data-test="password"]'))
      .withInput(
        "Repeat Password",
        By.css('div.form-group[data-test="password2"]')
      )
      .build();
  }
}
