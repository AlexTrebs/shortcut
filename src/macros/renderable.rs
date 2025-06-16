use axum::response::Html;
use tera::Tera;

/// `trait` for templates that require rendering from other files e.g. shared templates.
pub trait Renderable {
  fn get_html(&self, tera: Tera) -> Html<String>;
}

/// A macro to make a `renderable` implementation for the given struct to give pub access to rendering the template.
#[macro_export]
macro_rules! impl_renderable {
  ($($t:ty),*) => {
    $(impl Renderable for $t {
      fn get_html(&self, tera: Tera) -> Html<String> {
        Html(self.render(&tera))
      }
    })*
  };
}