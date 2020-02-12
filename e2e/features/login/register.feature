Feature: User Registration

  Scenario: Registering a user with no values
    Given I visit the home page
    And I start logging in as "unknown"
    And I am displayed the Register User page
    When I register with details:
      | Email Address     |  |
      | Display Name      |  |
      | Password          |  |
      | Re-enter Password |  |
    And the Register User form has errors:
      | Email Address     | Please enter an email address |
      | Display Name      | Please enter a display name   |
      | Password          | Please enter a password       |
      | Re-enter Password | Please re-enter the password  |
    And the Register User form has details:
      | Username          | unknown |
      | Email Address     |         |
      | Display Name      |         |
      | Password          |         |
      | Re-enter Password |         |

  Scenario Outline: Registering a user with invalid details: <Comment>
    Given I visit the home page
    And I start logging in as "unknown"
    And I am displayed the Register User page
    When I register with details:
      | Email Address     | <Email Address>     |
      | Display Name      | <Display Name>      |
      | Password          | <Password>          |
      | Re-enter Password | <Re-enter Password> |
    And the Register User form has errors:
      | Email Address     | <Email Address Error>     |
      | Display Name      | <Display Name Error>      |
      | Password          | <Password Error>          |
      | Re-enter Password | <Re-enter Password Error> |
    And the Register User form has details:
      | Username          | unknown         |
      | Email Address     | <Email Address> |
      | Display Name      | <Display Name>  |
      | Password          |                 |
      | Re-enter Password |                 |

    Examples:
      | Email Address    | Display Name | Password | Re-enter Password | Email Address Error                | Display Name Error          | Password Error | Re-enter Password Error | Comment             |
      | test@example.com | Test User    | Pa55word | password          |                                    |                             |                | Passwords do not match  | Different Passwords |
      | test@example.com | {space}      | Pa55word | Pa55word          |                                    | Please enter a display name |                |                         | No Display Name     |
      | test@example     | Test User    | Pa55word | Pa55word          | Please enter a valid email address |                             |                |                         | No Display Name     |

  @wip
  Scenario: Registering a user with a duplicate email address
    # Given a user exists with details:
    #   | Username      | known                |
    #   | Email Address | testuser@example.com |
    Given I visit the home page
    And I start logging in as "unknown"
    And I am displayed the Register User page
    When I register with details:
      | Email Address     | testuser@example.com |
      | Display Name      | Test User            |
      | Password          | Password             |
      | Re-enter Password | Password             |
    And the Register User form has errors:
      | Email Address | Email Address is already registered |
    And the Register User form has details:
      | Username          | unknown              |
      | Email Address     | testuser@example.com |
      | Display Name      | Test User            |
      | Password          | Password             |
      | Re-enter Password | Password             |
