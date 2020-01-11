@login
Feature: Start Authentication

  Scenario: Starting authentication with an unknown user displays the Register User form
    Given I visit the home page
    When I start logging in as "unknown"
    Then I am displayed the Register User form
    And the Register User form has details:
      | Username          | unknown |
      | Email Address     |         |
      | Display Name      |         |
      | Password          |         |
      | Re-enter Password |         |

  Scenario: Starting authentication with an unknown user displays the Register User form - username is padded
    Given I visit the home page
    When I start logging in as "  unknown  "
    Then I am displayed the Register User form
    And the Register User form has details:
      | Username          | unknown |
      | Email Address     |         |
      | Display Name      |         |
      | Password          |         |
      | Re-enter Password |         |

  Scenario: Starting authentication with an no username displays an error
    Given I visit the home page
    When I start logging in as "  "
    Then I am displayed the Start Login form
    And the Start Login form has errors:
      | Username | Please enter a username |

  Scenario Outline: Starting authentication with an unknown user displays the Register User form - username uses nasty characters: <Input>
    Given I visit the home page
    When I start logging in as "<Input>"
    Then I am displayed the Register User form
    And the Register User form has details:
      | Username          | <Expected> |
      | Email Address     |            |
      | Display Name      |            |
      | Password          |            |
      | Re-enter Password |            |

  Examples:
    | Input        | Expected     |
    | !@#$%^&*     | !@#$%^&*     |
    | Snow☃man     | Snow☃man     |
    | <b>hello</b> | <b>hello</b> |
    | \"quoted\"   | "quoted"     |
