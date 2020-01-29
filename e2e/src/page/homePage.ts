import { Url, PageName } from './pageModel';
import { BasePage } from './basePage';
import { WebElement } from 'selenium-webdriver';

/**
 * Page Object representing the home page of the application
 */
@PageName('home page')
@Url('/')
export class HomePage extends BasePage {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  async verifyCorrectPage() {
    return true;
  }
}
