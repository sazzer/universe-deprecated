Feature: User Profile

  Background: Create User
    Given a user already exists with details:
      | Username      | known                |
      | Email Address | testuser@example.com |
      | Display Name  | Test User            |
      | Password      | Pa55word             |

  Scenario: User Profile loads on login
    Given I visit the home page
    When I log in as "known" with password "Pa55word"
    Then I am displayed the User Profile page
    And the User Profile form has details:
      | Username      | known                |
      | Email Address | testuser@example.com |
      | Display Name  | Test User            |

  Scenario: Load the User Profile
    Given I visit the home page
    And I log in as "known" with password "Pa55word"
    And I go to the home page
    When I load the user profile
    Then I am displayed the User Profile page
    And the User Profile form has details:
      | Username      | known                |
      | Email Address | testuser@example.com |
      | Display Name  | Test User            |

  Scenario: Change User Profile successfully
    Given I visit the home page
    And I log in as "known" with password "Pa55word"
    When I update the User Profile form with details:
      | Email Address | newuser@example.com |
      | Display Name  | New User            |
    Then the User Profile form has details:
      | Username      | known               |
      | Email Address | newuser@example.com |
      | Display Name  | New User            |
    And I am logged in as "New User"

  Scenario: Change and reload Profile
    Given I visit the home page
    And I log in as "known" with password "Pa55word"
    When I update the User Profile form with details:
      | Email Address | newuser@example.com |
      | Display Name  | New User            |
    And I go to the home page
    When I load the user profile
    Then I am displayed the User Profile page
    And the User Profile form has details:
      | Username      | known               |
      | Email Address | newuser@example.com |
      | Display Name  | New User            |

  Scenario Outline: Change User Profile - Errors: <Comment>
    Given a user already exists with details:
      | Email Address | other@example.com |
    Given I visit the home page
    And I log in as "known" with password "Pa55word"
    When I update the User Profile form with details:
      | Email Address | <Email Address> |
      | Display Name  | <Display Name>  |
    Then the User Profile form has errors:
      | Email Address | <Email Address Error> |
      | Display Name  | <Display Name Error>  |
    And I am logged in as "Test User"

    Examples:
      | Email Address     | Display Name | Email Address Error                         | Display Name Error          | Comment                 |
      | {space}           | New User     | Please enter an email address               |                             | No Email Address        |
      | new@example.com   | {space}      |                                             | Please enter a display name | No Display Name         |
      | new@examplecom    | New User     | Please enter a valid email address          |                             | Malformed Email Address |
      | other@example.com | New User     | Email Address is registered to another user |                             | Duplicate Email Address |