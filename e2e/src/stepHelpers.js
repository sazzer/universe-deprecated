const { Then } = require('cucumber');
const { expect } = require('chai');
const { parseValue } = require('./dataTable');

/**
 * Build some standard cucumber steps for dealing with pages
 * @param  {string} name      The name of the page for the steps
 * @param  {Constructor} pageModel The page model to use
 */
function buildPageSteps(name, pageModel) {
  Then(`I am displayed the ${name} page`, async function() {
    await this.browser.buildPage(pageModel);
  });
}

/**
 * Build some standard cucumber steps for dealing with pages that contain forms
 * @param  {string} name      The name of the page for the steps
 * @param  {Constructor} pageModel The page model to use
 */
function buildFormSteps(name, pageModel) {
  Then(`the ${name} form has details:`, async function(data) {
    const page = await this.browser.buildPage(pageModel);
    const form = await page.getForm();
    const values = await form.getValues();

    const expected = data.rowsHash();
    const finalExpected = {};
    Object.keys(expected)
      .filter(key => expected[key])
      .forEach(key => finalExpected[key] = parseValue(expected[key]));

    expect(values).to.include(finalExpected);
  });

  Then(`the ${name} form has no errors`, async function() {
    const page = await this.browser.buildPage(pageModel);
    const form = await page.getForm();
    const values = await form.getErrors();

    expect(values).to.be.empty;
  });

  Then(`the ${name} form has errors:`, async function(data) {
    const page = await this.browser.buildPage(pageModel);
    const form = await page.getForm();
    const values = await form.getErrors();

    const expected = data.rowsHash();
    const finalExpected = {};
    Object.keys(expected)
      .filter(key => expected[key])
      .forEach(key => finalExpected[key] = parseValue(expected[key]));

    expect(values).to.include(finalExpected);
  });
}

module.exports = {
  buildPageSteps,
  buildFormSteps,
}
