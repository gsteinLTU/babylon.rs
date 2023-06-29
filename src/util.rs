/// Macro to add a getter and setter for a property of a JavaScript object
#[macro_export]
macro_rules! get_set_jsvalue {
    ($getter_name:ident, $setter_name:ident, $jsprop:expr, f64) => {
        pub fn $getter_name(&self) -> f64 {
            Reflect::get(&self, &JsValue::from_str($jsprop)).expect(&format!("{} not found", $jsprop)).unchecked_into_f64()
        }
    
        pub fn $setter_name(&self, val: f64) {
            Reflect::set(&self, &JsValue::from_str($jsprop), &JsValue::from_f64(val)).expect(&format!("{} not found", $jsprop));
        }
    };
    ($getter_name:ident, $setter_name:ident, $jsprop:expr, bool) => {
        pub fn $getter_name(&self) -> bool {
            Reflect::get(&self, &JsValue::from_str($jsprop)).expect(&format!("{} not found", $jsprop)).as_bool().unwrap()
        }
    
        pub fn $setter_name(&self, val: bool) {
            Reflect::set(&self, &JsValue::from_str($jsprop), &JsValue::from_bool(val)).expect(&format!("{} not found", $jsprop));
        }
    };
    ($getter_name:ident, $setter_name:ident, $jsprop:expr, $type:ty) => {
        pub fn $getter_name(&self) -> $type {
            Reflect::get(&self, &JsValue::from_str($jsprop)).expect(&format!("{} not found", $jsprop)).into()
        }
    
        pub fn $setter_name(&self, val: $type) {
            Reflect::set(&self, &JsValue::from_str($jsprop), &JsValue::from(val)).expect(&format!("{} not found", $jsprop));
        }
    };
}

pub(crate) use get_set_jsvalue as get_set_jsvalue;