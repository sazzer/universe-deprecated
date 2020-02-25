import React from "react";
import { Link, useRouteMatch, NavLink, Switch, Route } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { LoggedIn } from "../login/loggedIn";
import { UserProfileArea } from "./profile";

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
 * React Component represnting the user profile page
 */
export const ProfilePage: React.FC = () => {
  const { t } = useTranslation();

  const pages = [
    {
      name: "profile",
      path: "/profile",
      label: t("profile.profile.label"),
      content: <UserProfileArea />
    },
    {
      name: "worlds",
      path: "/profile/worlds",
      label: t("profile.worlds.label"),
      content: "My Worlds Content"
    },
    {
      name: "stories",
      path: "/profile/stories",
      label: t("profile.stories.label"),
      content: "My Stories Content"
    }
  ];

  const tabs = pages.map(page => (
    <ProfilePageTab
      key={page.name}
      path={page.path}
      name={page.name}
      label={page.label}
      content={page.content}
    />
  ));

  const panes = pages
    .reverse()
    .map(page => (
      <ProfilePagePane
        key={page.name}
        path={page.path}
        name={page.name}
        label={page.label}
        content={page.content}
      />
    ));

  return (
    <LoggedIn>
      <nav aria-label="breadcrumb">
        <ol className="breadcrumb">
          <li className="breadcrumb-item">
            <Link to="/">{t("profile.breadcrumbs.home")}</Link>
          </li>
          <li className="breadcrumb-item active" aria-current="page">
            {t("profile.breadcrumbs.profile")}
          </li>
        </ol>
      </nav>

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
    </LoggedIn>
  );
};
