// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkSocketAccessible")]
    pub struct SocketAccessible(Object<ffi::GtkSocketAccessible, ffi::GtkSocketAccessibleClass>) @extends atk::Object;

    match fn {
        type_ => || ffi::gtk_socket_accessible_get_type(),
    }
}

impl SocketAccessible {
    pub const NONE: Option<&'static SocketAccessible> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SocketAccessible`] objects.
    ///
    /// This method returns an instance of [`SocketAccessibleBuilder`](crate::builders::SocketAccessibleBuilder) which can be used to create [`SocketAccessible`] objects.
    pub fn builder() -> SocketAccessibleBuilder {
        SocketAccessibleBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SocketAccessible`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SocketAccessibleBuilder {
    accessible_description: Option<String>,
    accessible_name: Option<String>,
    accessible_parent: Option<atk::Object>,
    accessible_role: Option<atk::Role>,
    accessible_table_caption: Option<String>,
    accessible_table_caption_object: Option<atk::Object>,
    accessible_table_column_description: Option<String>,
    accessible_table_column_header: Option<atk::Object>,
    accessible_table_row_description: Option<String>,
    accessible_table_row_header: Option<atk::Object>,
    accessible_table_summary: Option<atk::Object>,
    accessible_value: Option<f64>,
}

impl SocketAccessibleBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`SocketAccessibleBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SocketAccessible`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SocketAccessible {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accessible_description) = self.accessible_description {
            properties.push(("accessible-description", accessible_description));
        }
        if let Some(ref accessible_name) = self.accessible_name {
            properties.push(("accessible-name", accessible_name));
        }
        if let Some(ref accessible_parent) = self.accessible_parent {
            properties.push(("accessible-parent", accessible_parent));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref accessible_table_caption) = self.accessible_table_caption {
            properties.push(("accessible-table-caption", accessible_table_caption));
        }
        if let Some(ref accessible_table_caption_object) = self.accessible_table_caption_object {
            properties.push((
                "accessible-table-caption-object",
                accessible_table_caption_object,
            ));
        }
        if let Some(ref accessible_table_column_description) =
            self.accessible_table_column_description
        {
            properties.push((
                "accessible-table-column-description",
                accessible_table_column_description,
            ));
        }
        if let Some(ref accessible_table_column_header) = self.accessible_table_column_header {
            properties.push((
                "accessible-table-column-header",
                accessible_table_column_header,
            ));
        }
        if let Some(ref accessible_table_row_description) = self.accessible_table_row_description {
            properties.push((
                "accessible-table-row-description",
                accessible_table_row_description,
            ));
        }
        if let Some(ref accessible_table_row_header) = self.accessible_table_row_header {
            properties.push(("accessible-table-row-header", accessible_table_row_header));
        }
        if let Some(ref accessible_table_summary) = self.accessible_table_summary {
            properties.push(("accessible-table-summary", accessible_table_summary));
        }
        if let Some(ref accessible_value) = self.accessible_value {
            properties.push(("accessible-value", accessible_value));
        }
        glib::Object::new::<SocketAccessible>(&properties)
    }

    pub fn accessible_description(mut self, accessible_description: &str) -> Self {
        self.accessible_description = Some(accessible_description.to_string());
        self
    }

    pub fn accessible_name(mut self, accessible_name: &str) -> Self {
        self.accessible_name = Some(accessible_name.to_string());
        self
    }

    pub fn accessible_parent(mut self, accessible_parent: &impl IsA<atk::Object>) -> Self {
        self.accessible_parent = Some(accessible_parent.clone().upcast());
        self
    }

    pub fn accessible_role(mut self, accessible_role: atk::Role) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn accessible_table_caption(mut self, accessible_table_caption: &str) -> Self {
        self.accessible_table_caption = Some(accessible_table_caption.to_string());
        self
    }

    pub fn accessible_table_caption_object(
        mut self,
        accessible_table_caption_object: &impl IsA<atk::Object>,
    ) -> Self {
        self.accessible_table_caption_object =
            Some(accessible_table_caption_object.clone().upcast());
        self
    }

    pub fn accessible_table_column_description(
        mut self,
        accessible_table_column_description: &str,
    ) -> Self {
        self.accessible_table_column_description =
            Some(accessible_table_column_description.to_string());
        self
    }

    pub fn accessible_table_column_header(
        mut self,
        accessible_table_column_header: &impl IsA<atk::Object>,
    ) -> Self {
        self.accessible_table_column_header = Some(accessible_table_column_header.clone().upcast());
        self
    }

    pub fn accessible_table_row_description(
        mut self,
        accessible_table_row_description: &str,
    ) -> Self {
        self.accessible_table_row_description = Some(accessible_table_row_description.to_string());
        self
    }

    pub fn accessible_table_row_header(
        mut self,
        accessible_table_row_header: &impl IsA<atk::Object>,
    ) -> Self {
        self.accessible_table_row_header = Some(accessible_table_row_header.clone().upcast());
        self
    }

    pub fn accessible_table_summary(
        mut self,
        accessible_table_summary: &impl IsA<atk::Object>,
    ) -> Self {
        self.accessible_table_summary = Some(accessible_table_summary.clone().upcast());
        self
    }

    pub fn accessible_value(mut self, accessible_value: f64) -> Self {
        self.accessible_value = Some(accessible_value);
        self
    }
}

pub trait SocketAccessibleExt: 'static {
    #[doc(alias = "gtk_socket_accessible_embed")]
    fn embed(&self, path: &str);
}

impl<O: IsA<SocketAccessible>> SocketAccessibleExt for O {
    fn embed(&self, path: &str) {
        unsafe {
            ffi::gtk_socket_accessible_embed(self.as_ref().to_glib_none().0, path.to_glib_none().0);
        }
    }
}

impl fmt::Display for SocketAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SocketAccessible")
    }
}
