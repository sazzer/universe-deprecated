import { Link } from "react-router-dom";
import React from "react";

/** Shape of a single breadcrump in the list, excluding the current page */
export interface Breadcrumb {
  link: string;
  label: string;
}

/** Props for the Breadcrumbs */
export interface BreadcrumbsProps {
  breadcrumbs: Breadcrumb[];
  currentLabel: string;
}

/**
 * Render some breadcrumbs for the page
 */
export const Breadcrumbs: React.FC<BreadcrumbsProps> = ({
  breadcrumbs,
  currentLabel
}) => {
  const links = breadcrumbs.map(({ link, label }, index) => {
    return (
      <li className="breadcrumb-item" key={`breadcrumb-${index}`}>
        <Link to={link}>{label}</Link>
      </li>
    );
  });

  return (
    <nav aria-label="breadcrumb">
      <ol className="breadcrumb">
        {links}
        <li className="breadcrumb-item active" aria-current="page">
          {currentLabel}
        </li>
      </ol>
    </nav>
  );
};
