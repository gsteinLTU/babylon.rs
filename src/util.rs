/// Macro to add a getter and setter for a property of a JavaScript object
/// 
/// Note that for f64, the type will need to be js_sys::Number
#[macro_export]
macro_rules! get_set_jsvalue {
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