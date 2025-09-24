// src/android.rs
use jni::{objects::{JObject, JClass, JString, GlobalRef}, JavaVM, JNIEnv, errors::Result as JniResult};
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::ffi::CString;

static JVM: OnceCell<JavaVM> = OnceCell::new();
static ACTIVITY: OnceCell<Mutex<GlobalRef>> = OnceCell::new();

/// Called from Java to initialize the native library with the Activity instance.
/// Java signature: public static native void init(Activity activity);
#[no_mangle]
pub extern "C" fn Java_com_example_smsgateway_NativeBridge_init(env: JNIEnv, _class: JClass, activity: JObject) {
    // store JavaVM
    match env.get_java_vm() {
        Ok(vm) => {
            let _ = JVM.set(vm);
        }
        Err(e) => {
            tracing::error!("Failed to get JavaVM: {:?}", e);
            return;
        }
    }

    // store global ref to activity
    match env.new_global_ref(activity) {
        Ok(gref) => {
            let _ = ACTIVITY.set(Mutex::new(gref));
            tracing::info!("Native init: activity global ref stored");
        }
        Err(e) => {
            tracing::error!("Failed to create global ref to activity: {:?}", e);
        }
    }
}

/// Send an SMS by calling Java's SmsHelper.sendSMS(Context ctx, String to, String msg)
pub fn send_sms(to: &str, message: &str) -> Result<(), String> {
    // Get JavaVM
    let vm = JVM.get().ok_or("JavaVM not initialized - call NativeBridge.init(activity) from Java".to_string())?;
    // Attach
    let mut env = vm.attach_current_thread().map_err(|e| format!("attach failed: {:?}", e))?;

    // Get activity global ref
    let activity_ref = ACTIVITY.get().ok_or("Activity not initialized".to_string())?;
    let guard = activity_ref.lock();
    let activity_obj = guard.as_obj();

    // Prepare Java strings
    let j_to = env.new_string(to).map_err(|e| format!("Failed to create jstring for 'to': {:?}", e))?;
    let j_msg = env.new_string(message).map_err(|e| format!("Failed to create jstring for 'message': {:?}", e))?;

    // Find SmsHelper class
    let class_name = "com/example/smsgateway/SmsHelper";
    let clazz = env.find_class(class_name).map_err(|e| format!("Failed to find SmsHelper class: {:?}", e))?;

    // Call static method: public static void sendSMS(Context ctx, String to, String msg)
    let method_id = env.get_static_method_id(clazz, "sendSMS", "(Landroid/content/Context;Ljava/lang/String;Ljava/lang/String;)V")
        .map_err(|e| format!("Failed to get sendSMS method id: {:?}", e))?;

    // Call
    match env.call_static_method_unchecked(clazz, method_id, jni::signature::ReturnType::Void, &[
        jni::objects::JValue::Object(activity_obj),
        jni::objects::JValue::Object(JObject::from(j_to)),
        jni::objects::JValue::Object(JObject::from(j_msg)),
    ]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("JNI call error: {:?}", e)),
    }
}
