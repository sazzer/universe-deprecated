const { BasePage } = require('./basepage');

/**
 * Page Model that represents the home page of the application
 * @extends BasePage
 */
class HomePage extends BasePage {
  constructor(element) {
    super(element);
  }

}

HomePage.URL = '/';

module.exports = { HomePage };
