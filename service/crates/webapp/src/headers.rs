use rocket::http::Header;
use std::fmt::Write;

// Header representation for `Accept-Patch`
pub struct AcceptPatch<H>(pub H)
where
  H: Into<String>;

impl<'h, H> From<AcceptPatch<H>> for Header<'h>
where
  H: Into<String>,
{
  fn from(header: AcceptPatch<H>) -> Self {
    Self::new("Accept-Patch", header.0.into())
  }
}

// Header representation for `Link`
pub struct Link {
  rel: Option<String>,
  href: String,
}

impl Link {
  // Construct a new `Link` header with an href
  pub fn from_href<H>(href: H) -> Self
  where
    H: Into<String>,
  {
    Self {
      rel: None,
      href: href.into(),
    }
  }

  // Assign a `rel` to the `Link` header
  pub fn with_rel<H>(self, rel: H) -> Self
  where
    H: Into<String>,
  {
    Self {
      rel: Some(rel.into()),
      ..self
    }
  }
}

impl<'h> From<Link> for Header<'h> {
  fn from(header: Link) -> Self {
    let mut value = String::new();
    write!(&mut value, "<{}>", header.href).unwrap();
    if let Some(rel) = header.rel {
      write!(&mut value, r#"; rel="{}""#, rel).unwrap()
    }
    Header::new("Link", value)
  }
}
