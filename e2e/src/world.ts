import { Before, After, HookScenarioResult, Status } from 'cucumber';
import { Browser } from './browser';

declare module "cucumber" {
  interface World {
    browser: Browser;
  }
}

Before(async function() {
  this.browser = new Browser();
  await this.browser.reset();
});

After(async function(scenario: HookScenarioResult) {
  const screenshot = await this.browser.screenshot();
  this.attach(screenshot, 'image/png');

  if (scenario.result.status === Status.FAILED) {
    this.browser.destroy();
  }
});
