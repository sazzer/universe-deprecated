Feature: Logging Out

  Scenario: Log out after authenticating as an existing user
    Given a user already exists with details:
      | Username     | known     |
      | Display Name | Test User |
      | Password     | Pa55word  |
    And I visit the home page
    And I start logging in as "known"
    And I am displayed the Authenticate User page
    And I authenticate with details:
      | Password | Pa55word |
    When I log out
    Then I am not logged in

  Scenario: Log out after registering as a new user
    Given I visit the home page
    And I start logging in as "unknown"
    And I am displayed the Register User page
    And I register with details:
      | Email Address     | testuser@example.com |
      | Display Name      | Test User            |
      | Password          | Password             |
      | Re-enter Password | Password             |
    When I log out
    Then I am not logged in
