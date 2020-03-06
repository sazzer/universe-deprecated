import { BasePageModel, PageModel } from "./pageModel";
import { WebElement, By } from "selenium-webdriver";
import { expect } from "chai";
import { Then, TableDefinition, When } from "cucumber";
import debug from "debug";
import { processObject } from "../table";
import { wait } from "../browser";

const LOG = debug("universe:e2e:form");

interface FormField {
  selector: By;
  setter: (value: string) => Promise<any>;
  getter: () => Promise<string>;
  errorGetter?: () => Promise<string | undefined>;
}

/**
 * Page Model for working with forms
 */
export class FormModel extends BasePageModel {
  /** The form fields */
  private _fields: { [name: string]: FormField };

  constructor(baseElement: WebElement, fields: { [name: string]: FormField }) {
    super(baseElement);
    this._fields = fields;
  }

  /**
   * Set the value of the field identified by the given name
   * @param  name  The name to identify the field
   * @param  value The new value for the field
   */
  async setField(name: string, value: string) {
    LOG('Setting field %s to value "%s"', name, value);
    expect(this._fields).to.haveOwnProperty(name);

    const fieldDefinition = this._fields[name];
    await fieldDefinition.setter(value);
  }

  /**
   * Get the value of the field with the given name
   * @param  name The name of the field
   */
  async getField(name: string) {
    LOG("Getting value of field: %s", name);
    const field = this._fields[name];
    const result = await field.getter();
    LOG("Retrieved value of field %s: %s", name, result);
    return result;
  }

  /**
   * Get the error of the field with the given name
   * @param  name The name of the field
   */
  async getError(name: string) {
    LOG("Getting error of field: %s", name);
    const field = this._fields[name];
    let result;
    if (field.errorGetter) {
      result = await field.errorGetter();
    }
    LOG("Retrieved error of field %s: %s", name, result);
    return result;
  }

  /**
   * Get all of the values from the form
   */
  async getAllValues(): Promise<{ [value: string]: string }> {
    const result: { [value: string]: string } = {};
    for (const name of Object.keys(this._fields)) {
      result[name] = await this.getField(name);
    }

    return result;
  }

  /**
   * Set all of the values on the form
   */
  async setAllValues(input: { [name: string]: string }) {
    for (const name of Object.keys(input)) {
      await this.setField(name, input[name]);
    }
  }

  /**
   * Get all of the errors from the form
   */
  async getAllErrors(): Promise<{ [value: string]: string }> {
    const result: { [value: string]: string } = {};
    for (const name of Object.keys(this._fields)) {
      const error = await this.getError(name);
      if (error) {
        result[name] = error;
      }
    }

    return result;
  }

  /**
   * Submit the form
   */
  async submit() {
    const submitButton = await this.findElement(
      By.css('button[type="submit"]')
    );
    await submitButton.click();

    await wait(async () => {
      try {
        const disabled = await submitButton.getAttribute("disabled");
        return disabled !== "true";
      } catch (e) {
        if (e.name === "StaleElementReferenceError") {
          return true;
        }
        throw e;
      }
    });
  }
}

/**
 * Builder to make it easier to work with Form Models
 */
export class FormModelBuilder {
  /** The base element */
  private _baseElement: WebElement;

  /** The form fields */
  private _fields: { [name: string]: FormField } = {};

  /**
   * Construct the builder
   * @param baseElement The base element
   */
  constructor(baseElement: WebElement) {
    this._baseElement = baseElement;
  }

  withInput(name: string, selector: By) {
    this._fields[name] = {
      selector,
      setter: async (value: string) => {
        const element = await this._baseElement.findElement(selector);
        const input = await element.findElement(By.tagName("input"));
        await input.clear();
        await input.sendKeys(value);
      },
      getter: async () => {
        const element = await this._baseElement.findElement(selector);
        const input = await element.findElement(By.tagName("input"));
        return await input.getAttribute("value");
      },
      errorGetter: async () => {
        const element = await this._baseElement.findElement(selector);
        const input = await element.findElement(By.tagName("input"));
        const cssClasses = await input.getAttribute("class");
        if (cssClasses.split(" ").includes("is-invalid")) {
          const error = await element.findElement(
            By.className("invalid-feedback")
          );
          return await error.getText();
        } else {
          return undefined;
        }
      }
    };
    return this;
  }

  /**
   * Build the Form Model
   */
  build() {
    return new FormModel(this._baseElement, this._fields);
  }
}

export interface FormPageModel extends PageModel {
  getForm(): Promise<FormModel>;
}

export interface FormPageModelConstructor<T extends FormPageModel> {
  new (driver: WebElement): T;
}

export function FormPage(name: string) {
  return function<T extends FormPageModel>(
    constructor: FormPageModelConstructor<T>
  ) {
    LOG(
      'Building form steps with name "%s" for page model: %o',
      name,
      constructor
    );

    Then(`the ${name} form has details:`, async function(
      dataTable: TableDefinition
    ) {
      const page = await this.browser.newPageModel(constructor);
      const form = await page.getForm();
      const values = await form.getAllValues();

      const expected = dataTable.rowsHash();
      expect(values).to.contain(processObject(expected));
    });

    Then(`the ${name} form has no errors`, async function() {
      const page = await this.browser.newPageModel(constructor);
      const form = await page.getForm();
      const values = await form.getAllErrors();
      expect(values).to.be.empty;
    });

    Then(`the ${name} form has errors:`, async function(
      dataTable: TableDefinition
    ) {
      const page = await this.browser.newPageModel(constructor);
      const form = await page.getForm();
      const values = await form.getAllErrors();

      const expected = dataTable.rowsHash();
      expect(values).to.contain(processObject(expected));
    });

    When(`I update the ${name} form with details:`, async function(
      dataTable: TableDefinition
    ) {
      const page = await this.browser.newPageModel(constructor);
      const form = await page.getForm();

      await form.setAllValues(processObject(dataTable.rowsHash()));
      await form.submit();
    });
  };
}
