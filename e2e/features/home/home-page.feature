@ignore
Feature: Load the home page

  Scenario: Load the home page
    When I visit the home page
    Then I am not logged in
