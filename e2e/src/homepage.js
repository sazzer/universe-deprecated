const { When } = require('cucumber');
const { HomePage } = require('./pages/homepage');

When('I visit the home page', async function() {
  await this.browser.visit(HomePage);
});
