const { By } = require('selenium-webdriver');

/**
* Page element to represent a form so that the fields can be accessed
*/
class FormPage {
  constructor(element, fields) {
    this._element = element;
    this._fields = fields;
  }

  /**
   * Get the contents of the form as currently displayed
   */
  async getValues() {
    const data = {};
    const fields = this._fields;
    for (const field of Object.keys(fields)) {
      const selector = fields[field];
      const element = await this._element.findElement(By.css(`${selector} input`));
      const value = await element.getAttribute('value');
      data[field] = value;
    }

    return data;
  }
}

module.exports = { FormPage };
