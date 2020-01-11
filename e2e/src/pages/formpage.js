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

  /**
   * Get the errors from the form as currently displayed
   */
  async getErrors() {
    const errors = {};
    const fields = this._fields;
    for (const field of Object.keys(fields)) {
      const selector = fields[field];
      const input = await this._element.findElement(By.css(`${selector} input`));
      const inputClasses = await input.getAttribute('class');
      if (inputClasses.split(' ').includes('is-invalid')) {
        const error = await this._element.findElement(By.css(`${selector} div.invalid-feedback`));

        const value = await error.getText();
        errors[field] = value;
      }
    }

    return errors;
  }
}

module.exports = { FormPage };
