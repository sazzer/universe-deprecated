import { Link, NavLink, Route, Switch, useRouteMatch } from "react-router-dom";

import { Breadcrumbs } from "../components/breadcrumbs";
import { LoggedIn } from "../loggedIn";
import React from "react";
import { useTranslation } from "react-i18next";

/** Props to represent an entry in the profile page */
interface ProfilePageEntry {
  name: string;
  path: string;
  label: string;
  content: any;
}

/** Component to represent a single tab on the profile page */
const ProfilePageTab: React.FC<ProfilePageEntry> = ({ path, name, label }) => {
  const match = useRouteMatch({ path: path, exact: true });

  return (
    <NavLink
      to={path}
      exact
      className="nav-link"
      id={`profile-tabs-${name}-tab`}
      role="tab"
      aria-controls={`profile-tabs-${name}`}
      aria-selected={match !== null}
    >
      {label}
    </NavLink>
  );
};

/** Component to represent the actual body of the profile page */
const ProfilePagePane: React.FC<ProfilePageEntry> = ({
  path,
  name,
  content
}) => {
  return (
    <Route exact path={path}>
      <div
        aria-labelledby={`profile-tabs-${name}-tab`}
        role="tabpanel"
        id={`profile-tabs-${name}`}
      >
        {content}
      </div>
    </Route>
  );
};

/**
 * Page representing the user profile
 */
export const ProfilePage: React.FC = () => {
  const { t } = useTranslation();

  const pages = [
    {
      name: "profile",
      path: "/profile",
      label: t("profile.profile.label"),
      content: "User Profile Area"
    },
    {
      name: "password",
      path: "/profile/password",
      label: t("profile.password.label"),
      content: "Change Password"
    }
  ];

  const tabs = pages.map(page => <ProfilePageTab {...page} />);

  const panes = pages.reverse().map(page => <ProfilePagePane {...page} />);

  return (
    <LoggedIn>
      <div data-test="profilePage">
        <Breadcrumbs
          currentLabel={t("profile.breadcrumbs.profile")}
          breadcrumbs={[{ link: "/", label: t("profile.breadcrumbs.home") }]}
        />

        <div className="row">
          <div className="col-12 col-md-9 order-sm-1">
            <div className="tab-content">
              <Switch>{panes}</Switch>
            </div>
          </div>

          <div className="col-12 col-md-3">
            <div
              className="nav flex-column nav-pills"
              id="profileSupportedContent"
              role="tablist"
              aria-orientation="vertical"
            >
              {tabs}
            </div>
          </div>
        </div>
      </div>
    </LoggedIn>
  );
};
