import { BasePageModel } from "./pageModel";
import { WebElement, By } from "selenium-webdriver";

export class HeaderPageModel extends BasePageModel {
  constructor(baseElement: WebElement) {
    super(baseElement);
  }

  /**
   * Check if we're currently logged in or not
   * @return True if we're logged in. False if not.
   */
  async isLoggedIn() {
    const loginLink = await this.findElement(
      By.css('a.nav-link[href="/login"]')
    );
    const loginLinkPresent = await loginLink.isDisplayed();
    return !loginLinkPresent;
  }

  async loggedInUser() {
    const userMenu = await this.findElement(
      By.css(
        'li.nav-item.dropdown a.nav-link.dropdown-toggle[data-toggle="dropdown"]'
      )
    );

    return await userMenu.getText();
  }

  async logout() {
    const userMenu = await this.findElement(
      By.css(
        'li.nav-item.dropdown a.nav-link.dropdown-toggle[data-toggle="dropdown"]'
      )
    );

    await userMenu.click();

    const logout = await this.findElement(
      By.css(
        "li.nav-item.dropdown div.dropdown-menu a.dropdown-item[href='/login']"
      )
    );
    await logout.click();
  }

  /**
   * Start the login process
   */
  async startLogin() {
    const loginLink = await this.findElement(
      By.css('a.nav-link[href="/login"]')
    );
    await loginLink.click();
  }
}
