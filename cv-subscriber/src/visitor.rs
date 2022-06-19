use crate::utils::*;
use crate::*;
use std::collections::HashMap;
use std::fmt;
use tracing_core::{field::Visit, span, Field};
use valuable::Value;

pub struct ImageVisitor {
    id: Option<span::Id>,
    images: HashMap<Field, u64>,
}

impl ImageVisitor {
    pub fn new(root: Option<span::Id>) -> Self {
        Self {
            id: root,
            images: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.images.is_empty()
    }
}

impl Visit for ImageVisitor {
    fn record_value(&mut self, field: &Field, value: valuable::Value<'_>) {
        if is_image(&value) {
            // Urgh I think I want to do an interner thingy where I pop images in and if they
            // didn't get exist I get an ID and then I just store that Id in places
            self.images.insert(field.clone(), 0);
        }
    }

    fn record_f64(&mut self, field: &Field, value: f64) {}

    fn record_i64(&mut self, field: &Field, value: i64) {}

    fn record_u64(&mut self, field: &Field, value: u64) {}

    fn record_bool(&mut self, field: &Field, value: bool) {}

    fn record_str(&mut self, field: &Field, value: &str) {}

    fn record_error(&mut self, _field: &Field, _value: &(dyn std::error::Error + 'static)) {}

    fn record_debug(&mut self, _field: &Field, _value: &dyn fmt::Debug) {}
}
