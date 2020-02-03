import { Url, PageName } from '../pageModel';
import { BasePage } from '../basePage';
import { FormPage, FormModelBuilder } from '../form';
import { WebElement, By } from 'selenium-webdriver';

/**
 * Page Object representing the register user of the application
 */
@PageName('Register User')
@FormPage('Register User')
@Url('/login')
export class RegisterUserPage extends BasePage {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  async verifyCorrectPage() {
    const form = await this.findElement(By.css('form[data-test="registerForm"]'));
    return await form.isDisplayed();
  }

  async getForm() {
    const form = await this.findElement(By.css('form[data-test="registerForm"]'));

    return new FormModelBuilder(form)
      .withInput('Username', By.css('div.form-group[data-test="username"]'))
      .withInput('Email Address', By.css('div.form-group[data-test="email"]'))
      .withInput('Display Name', By.css('div.form-group[data-test="displayName"]'))
      .withInput('Password', By.css('div.form-group[data-test="password"]'))
      .withInput('Re-enter Password', By.css('div.form-group[data-test="password2"]'))
      .build();
  }
}
