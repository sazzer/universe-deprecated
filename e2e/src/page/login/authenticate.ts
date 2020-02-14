import { Url, PageName } from "../pageModel";
import { BasePage } from "../basePage";
import { FormPage, FormModelBuilder } from "../form";
import { WebElement, By } from "selenium-webdriver";

/**
 * Page Object to authenticate a known user
 */
@PageName("Authenticate User")
@FormPage("Authenticate User")
@Url("/login")
export class AuthenticateUserPage extends BasePage {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  async verifyCorrectPage() {
    const form = await this.findElement(
      By.css('form[data-test="authenticateForm"]')
    );
    return await form.isDisplayed();
  }

  async getForm() {
    const form = await this.findElement(
      By.css('form[data-test="authenticateForm"]')
    );

    return new FormModelBuilder(form)
      .withInput("Username", By.css('div.form-group[data-test="username"]'))
      .withInput("Password", By.css('div.form-group[data-test="password"]'))
      .build();
  }
}
