#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = EXT_disjoint_timer_query_webgl2 , typescript_type = "EXT_disjoint_timer_query_webgl2")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtDisjointTimerQueryWebGl2` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub type ExtDisjointTimerQueryWebGl2;
    #[cfg(feature = "WebGlQuery")]
    # [wasm_bindgen (method , structural , js_class = "EXT_disjoint_timer_query_webgl2" , js_name = beginQueryEXT)]
    #[doc = "The `beginQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2/beginQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`, `WebGlQuery`*"]
    pub fn begin_query_ext(this: &ExtDisjointTimerQueryWebGl2, target: u32, query: &WebGlQuery);
    #[cfg(feature = "WebGlQuery")]
    # [wasm_bindgen (method , structural , js_class = "EXT_disjoint_timer_query_webgl2" , js_name = createQueryEXT)]
    #[doc = "The `createQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2/createQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`, `WebGlQuery`*"]
    pub fn create_query_ext(this: &ExtDisjointTimerQueryWebGl2) -> Option<WebGlQuery>;
    #[cfg(feature = "WebGlQuery")]
    # [wasm_bindgen (method , structural , js_class = "EXT_disjoint_timer_query_webgl2" , js_name = deleteQueryEXT)]
    #[doc = "The `deleteQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2/deleteQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`, `WebGlQuery`*"]
    pub fn delete_query_ext(this: &ExtDisjointTimerQueryWebGl2, query: Option<&WebGlQuery>);
    # [wasm_bindgen (method , structural , js_class = "EXT_disjoint_timer_query_webgl2" , js_name = endQueryEXT)]
    #[doc = "The `endQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2/endQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub fn end_query_ext(this: &ExtDisjointTimerQueryWebGl2, target: u32);
    # [wasm_bindgen (method , structural , js_class = "EXT_disjoint_timer_query_webgl2" , js_name = getQueryEXT)]
    #[doc = "The `getQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2/getQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub fn get_query_ext(
        this: &ExtDisjointTimerQueryWebGl2,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlQuery")]
    # [wasm_bindgen (method , structural , js_class = "EXT_disjoint_timer_query_webgl2" , js_name = getQueryObjectEXT)]
    #[doc = "The `getQueryObjectEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2/getQueryObjectEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`, `WebGlQuery`*"]
    pub fn get_query_object_ext(
        this: &ExtDisjointTimerQueryWebGl2,
        query: &WebGlQuery,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlQuery")]
    # [wasm_bindgen (method , structural , js_class = "EXT_disjoint_timer_query_webgl2" , js_name = isQueryEXT)]
    #[doc = "The `isQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2/isQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`, `WebGlQuery`*"]
    pub fn is_query_ext(this: &ExtDisjointTimerQueryWebGl2, query: Option<&WebGlQuery>) -> bool;
    #[cfg(feature = "WebGlQuery")]
    # [wasm_bindgen (method , structural , js_class = "EXT_disjoint_timer_query_webgl2" , js_name = queryCounterEXT)]
    #[doc = "The `queryCounterEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2/queryCounterEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`, `WebGlQuery`*"]
    pub fn query_counter_ext(this: &ExtDisjointTimerQueryWebGl2, query: &WebGlQuery, target: u32);
}
impl ExtDisjointTimerQueryWebGl2 {
    #[doc = "The `EXT_disjoint_timer_query_webgl2.QUERY_COUNTER_BITS_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub const QUERY_COUNTER_BITS_EXT: u32 = 34916u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query_webgl2.CURRENT_QUERY_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub const CURRENT_QUERY_EXT: u32 = 34917u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query_webgl2.QUERY_RESULT_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub const QUERY_RESULT_EXT: u32 = 34918u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query_webgl2.QUERY_RESULT_AVAILABLE_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub const QUERY_RESULT_AVAILABLE_EXT: u32 = 34919u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query_webgl2.TIME_ELAPSED_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub const TIME_ELAPSED_EXT: u32 = 35007u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query_webgl2.TIMESTAMP_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub const TIMESTAMP_EXT: u32 = 36392u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query_webgl2.GPU_DISJOINT_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQueryWebGl2`*"]
    pub const GPU_DISJOINT_EXT: u32 = 36795u64 as u32;
}
