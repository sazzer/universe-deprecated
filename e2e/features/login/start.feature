@ignore
Feature: Start Authentication

  Scenario: Starting authentication with an unknown user displays the Register User form
    Given I visit the home page
    When I start logging in as "unknown"
    Then I am displayed the Register User page
    And the Register User form has details:
      | Username          | unknown |
      | Email Address     |         |
      | Display Name      |         |
      | Password          |         |
      | Re-enter Password |         |

  Scenario: Starting authentication with an unknown user displays the Register User form - username is padded
    Given I visit the home page
    When I start logging in as "  unknown  "
    Then I am displayed the Register User page
    And the Register User form has details:
      | Username          | unknown |
      | Email Address     |         |
      | Display Name      |         |
      | Password          |         |
      | Re-enter Password |         |

  Scenario: Starting authentication with a known user displays the Login User form
    Given a user exists with details:
      | Username | known |
    And I visit the home page
    When I start logging in as "known"
    Then I am displayed the Login User page
    And the Login User form has details:
      | Username          | known |
      | Password          |       |

  Scenario: Starting authentication with a known user displays the Login User form - username is padded
    Given a user exists with details:
      | Username | known |
    And I visit the home page
    When I start logging in as "  known  "
    Then I am displayed the Login User page
    And the Login User form has details:
      | Username          | known |
      | Password          |       |

  @wip
  Scenario: Starting authentication with an a blank username displays an error
    Given I visit the home page
    When I start logging in as ""
    Then I am displayed the Start Login page
    And the Start Login form has details:
      | Username | |
    And the Start Login form has errors:
      | Username | Please enter a username |

  Scenario: Starting authentication with an a whitespace username displays an error
    Given I visit the home page
    When I start logging in as "  "
    Then I am displayed the Start Login page
    And the Start Login form has details:
      | Username          | |
    And the Start Login form has errors:
      | Username | Please enter a username |

  Scenario Outline: Starting authentication with an unknown user displays the Register User form - username uses nasty characters: <Input>
    Given I visit the home page
    When I start logging in as "<Input>"
    Then I am displayed the Register User page
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

  Scenario Outline: Starting authentication with a known user displays the Login User form - username uses nasty characters: <Input>
    Given a user exists with details:
      | Username | <Expected> |
    And I visit the home page
    When I start logging in as "<Input>"
    Then I am displayed the Login User page
    And the Login User form has details:
      | Username          | <Expected> |
      | Password          |            |

    Examples:
      | Input        | Expected     |
      | !@#$%^&*     | !@#$%^&*     |
      | Snow☃man     | Snow☃man     |
      | <b>hello</b> | <b>hello</b> |
      | \"quoted\"   | "quoted"     |
