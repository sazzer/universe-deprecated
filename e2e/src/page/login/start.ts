import { Url, PageName } from '../pageModel';
import { BasePage } from '../basePage';
import { FormPage, FormModelBuilder } from '../form';
import { WebElement, By } from 'selenium-webdriver';

/**
 * Page Object representing the login page of the application
 */
@PageName('Start Login')
@FormPage('Start Login')
@Url('/login')
export class StartLoginPage extends BasePage {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  async verifyCorrectPage() {
    const form = await this.findElement(By.css('form[data-test="startLoginForm"]'));
    return await form.isDisplayed();
  }

  async getForm() {
    const form = await this.findElement(By.css('form[data-test="startLoginForm"]'));

    return new FormModelBuilder(form)
      .withInput('Username', By.css('div.form-group[data-test="username"]'))
      .build();
  }
}
