use crate::utils::*;
use std::collections::HashMap;
use std::fmt;
use tracing_core::field::Visit;
use tracing_core::Field;
use valuable::Value;

#[derive(Clone, Default)]
pub struct ImageVisitor {
    images: HashMap<Field, ()>,
}

impl Visit for ImageVisitor {
    fn record_value(&mut self, field: &Field, value: valuable::Value<'_>) {
        if is_image(&value) {
            self.images.insert(field.clone(), ());
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
