Feature: User Authentication

  Background: Create User
    Given a user already exists with details:
      | Username     | known     |
      | Display Name | Test User |
      | Password     | Pa55word  |

  Scenario: Authenticating a user with no values
    Given I visit the home page
    And I start logging in as "known"
    And I am displayed the Authenticate User page
    When I authenticate with details:
      | Password |  |
    Then I am displayed the Authenticate User page
    And the Authenticate User form has errors:
      | Password | Please enter a password |
    And the Authenticate User form has details:
      | Username | known |
      | Password |       |
    And I am not logged in

  Scenario: Authenticating a user with the wrong password
    Given I visit the home page
    And I start logging in as "known"
    And I am displayed the Authenticate User page
    When I authenticate with details:
      | Password | wrong |
    Then I am displayed the Authenticate User page
    And the Authenticate User form has errors:
      | Password | Invalid username or password |
    And the Authenticate User form has details:
      | Username | known |
      | Password | wrong |
    And I am not logged in

  Scenario: Authenticating a user with the correct password
    Given I visit the home page
    And I start logging in as "known"
    And I am displayed the Authenticate User page
    When I authenticate with details:
      | Password | Pa55word |
    Then I am logged in as "Test User"
    And I am displayed the User Profile page