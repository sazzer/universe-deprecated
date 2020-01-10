const reporter = require('cucumber-html-reporter');

var options = {
  theme: 'foundation',
  jsonFile: 'output/cucumber_report.json',
  output: 'output/cucumber_report.html',
  reportSuiteAsScenarios: true,
  scenarioTimestamp: true,
  launchReport: false,
  storeScreenshots: true,
  screenshotsDirectory: 'output/screenshots',
  noInlineScreenshots: true,
};

reporter.generate(options);
