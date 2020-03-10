Feature: Change Password

  Background: Create User
    Given a user already exists with details:
      | Username      | known                |
      | Email Address | testuser@example.com |
      | Display Name  | Test User            |
      | Password      | Pa55word             |

  Scenario: Change Password successfully
    Given I visit the home page
    And I log in as "known" with password "Pa55word"
    When I open the Change Password form
    When I update the Change Password form with details:
      | New Password    | password123 |
      | Repeat Password | password123 |
    Then the Change Password form has details:
      | New Password    |  |
      | Repeat Password |  |

  Scenario: Log in again after changing password
    Given I visit the home page
    And I log in as "known" with password "Pa55word"
    And I open the Change Password form
    And I update the Change Password form with details:
      | New Password    | password123 |
      | Repeat Password | password123 |
    And the Change Password form has details:
      | New Password    |  |
      | Repeat Password |  |
    When I log out
    And I log in as "known" with password "password123"
    Then I am logged in as "Test User"

  Scenario Outline: Change Password - Errors: <Comment>
    Given I visit the home page
    And I log in as "known" with password "Pa55word"
    When I open the Change Password form
    When I update the Change Password form with details:
      | New Password    | <New Password>    |
      | Repeat Password | <Repeat Password> |
    Then the Change Password form has errors:
      | New Password    | <New Password Error>    |
      | Repeat Password | <Repeat Password Error> |

    Examples:
      | New Password | Repeat Password | New Password Error      | Repeat Password Error        | Comment               |
      | {blank}      | {blank}         | Please enter a password | Please re-enter the password | Both fields blank     |
      | password123  | {blank}         |                         | Passwords do not match       | Repeat Password blank |
      | {blank}      | password123     | Please enter a password | Passwords do not match       | New Password blank    |
      | Password321  | password123     |                         | Passwords do not match       | Password mismatch     |