use std::ffi::CStr;
use std::ptr::NonNull;
use arkui_sys::native_gesture::{ArkUI_GestureRecognizer, ArkUI_NativeGestureAPI_1};
use arkui_sys::native_interface::{ArkUI_NativeAPIVariantKind, OH_ArkUI_QueryModuleInterfaceByName};

static ARK_UI_NATIVE_GESTURE_API_1: &CStr = c"ARK_UI_NATIVE_GESTURE_API_1";
pub enum GestureApiError {
    QueryModuleInterfaceFailed,
}

pub enum CreateGestureError {
    /// The `GestureApi` did not contain a valid function pointer to create the requested
    /// gesture.
    ApiUnavailable,
    /// The create gesture function failed.
    Failed,
}

pub enum NumFingers {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

struct GestureApi(NonNull<ArkUI_NativeGestureAPI_1>);

// Todo: Do we need to handle / register ArkUI_GestureRecognizerDisposeNotifyCallback ?
struct GestureRecognizer(NonNull<ArkUI_GestureRecognizer>);

impl GestureApi {
    pub fn new() -> Result<GestureApi, GestureApiError> {
        // SAFETY: We pass in valid inputs and validate the pointer is non-zero before casting to
        // the same type `ArkUI_NativeGestureAPI_1` that we requested via the call.
        // Todo: The lifetime of the pointer and if it needs to be freed by us is not specified by
        // the documentation and needs to be clarified.
        // For now we assume it has a static lifetime and never needs to be freed.
        unsafe {
            let api = OH_ArkUI_QueryModuleInterfaceByName(ArkUI_NativeAPIVariantKind::ARKUI_NATIVE_GESTURE, ARK_UI_NATIVE_GESTURE_API_1.as_ptr());
            NonNull::new(api as *mut ArkUI_NativeGestureAPI_1)
                .map( GestureApi)
                .ok_or(GestureApiError::QueryModuleInterfaceFailed)
        }
    }

    /// Create a tap gesture with one, two or more taps and requiring one or more fingers
    ///
    /// ## consecutive_taps
    ///
    /// * If multiple taps are configured, each consecutive tap must happen within 300 ms to be
    ///   detected.
    /// * If the distance between the last tapped position and the current tapped position exceeds
    ///   60 virtual pixels, gesture recognition fails.
    ///
    ///  ## min_fingers
    ///
    /// * If the value is greater than 1, the tap gesture will fail to be recognized when the
    ///   number of fingers touching the screen within 300 ms of the first finger touch is less
    ///   than the required number, or when the number of fingers lifted from the screen within
    ///   300 ms of the first finger's being lifted is less than the required number.
    /// * When the number of fingers touching the screen **exceeds** the set value,
    ///   the gesture can be recognized.
    pub fn create_tap_gesture(&self, consecutive_taps: u8, min_fingers: NumFingers ) -> Result<GestureRecognizer, CreateGestureError> {
        // SAFETY: We assume the pointer to teh API is valid until self is dropped, but this
        // needs to be clarified. `See GestureApi::new()`
        if let Some(create_tap_gesture) = unsafe { (*self.0.as_ptr()).createTapGesture } {
            let gesture = unsafe {create_tap_gesture(consecutive_taps as i32, min_fingers as i32)};
            NonNull::new(gesture as *mut ArkUI_GestureRecognizer)
                .map(GestureRecognizer)
                .ok_or(CreateGestureError::Failed)
        } else {
            Err(CreateGestureError::ApiUnavailable)
        }
    }
}

impl GestureRecognizer {
    pub fn gesture_type(&self) {
        let recognizer = self.0.as_ptr();
        todo!()

    }
}