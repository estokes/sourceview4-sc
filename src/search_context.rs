use crate::SearchContext;
use glib::translate::*;
use glib::IsA;

pub trait SearchContextExtManual: 'static {
    #[doc(alias = "gtk_source_search_context_replace_all")]
    fn replace_all(&self, replace: &str) -> Result<(), glib::Error>;
}

impl<O: IsA<SearchContext>> SearchContextExtManual for O {
    fn replace_all(&self, replace: &str) -> Result<(), glib::Error> {
        let replace_length = replace.len() as i32;
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gtk_source_search_context_replace_all(
                self.as_ref().to_glib_none().0,
                replace.to_glib_none().0,
                replace_length,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
