// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping from the
// current local frame (which is the scope within which local (temporary)
// references to Java objects remain valid)
use jni::objects::{JClass, JString};
use jni::sys::jint;

// use jni::objects::JByteArray;
// use jni::sys::{jint, jlong};

const GLOBAL_VERSION_STRING: &'static str = "0.99.2-RS";

// This `#[no_mangle]` keeps rust from "mangling" the name and making it unique
// for this crate. The name follow a strict naming convention so that the
// JNI implementation will be able to automatically find the implementation
// of a native method based on its name.
//
// The `'local` lifetime here represents the local frame within which any local
// (temporary) references to Java objects will remain valid.
//
// It's usually not necessary to explicitly name the `'local` input lifetimes but
// in this case we want to return a reference and show the compiler what
// local frame lifetime it is associated with.
//
// Alternatively we could instead return the `jni::sys::jstring` type instead
// which would represent the same thing as a raw pointer, without any lifetime,
// and at the end use `.into_raw()` to convert a local reference with a lifetime
// into a raw pointer.
#[no_mangle]
pub extern "system" fn Java_com_zoffcc_applications_jninotifications_NTFYActivity_jninotifications_1version<'local>(
    // Notice that this `env` argument is mutable. Any `JNIEnv` API that may
    // allocate new object references will take a mutable reference to the
    // environment.
    env: JNIEnv<'local>,
    // this is the class that owns our static method. Not going to be used, but
    // still needs to have an argument slot
    _class: JClass<'local>,
    // input: JString<'local>,
) -> JString<'local> {
    env.new_string(GLOBAL_VERSION_STRING)
        .expect("Couldn't create java string!")
}

#[no_mangle]
pub extern "system" fn Java_com_zoffcc_applications_jninotifications_NTFYActivity_jninotifications_1notify<'local>(
    // Notice that this `env` argument is mutable. Any `JNIEnv` API that may
    // allocate new object references will take a mutable reference to the
    // environment.
    mut env: JNIEnv<'local>,
    // this is the class that owns our static method. Not going to be used, but
    // still needs to have an argument slot
    _class: JClass<'local>,
    application: JString<'local>,
    title: JString<'local>,
    message: JString<'local>,
    icon_filename_fullpath: JString<'local>,
) -> jint {
    if application.is_null() {
        eprintln!("JString application was null!");
        return -1;
    }
    if title.is_null() {
        eprintln!("JString title was null!");
        return -1;
    }
    if message.is_null() {
        eprintln!("JString message was null!");
        return -1;
    }

    let app_name: String = match env.get_string(&application) {
        Ok(v) => v.into(),
        Err(e) => {
            eprintln!("couldn't read JString application! {e}");
            return -1;
        }
    };
    let summary: String = match env.get_string(&title) {
        Ok(v) => v.into(),
        Err(e) => {
            eprintln!("couldn't read JString title! {e}");
            return -1;
        }
    };
    let body: String = match env.get_string(&message) {
        Ok(v) => v.into(),
        Err(e) => {
            eprintln!("couldn't read JString message! {e}");
            return -1;
        }
    };

    let mut ret = 0;

    // Init libnotify
    if let Err(e) = libnotify::init(&app_name) {
        eprintln!("libnotify::init failed! {e}");
        ret = -3;
    }

    let n: libnotify::Notification;

    if !icon_filename_fullpath.is_null() {
        let icon: String = match env.get_string(&icon_filename_fullpath) {
            Ok(v) => v.into(),
            Err(_) => return -1,
        };
        n = libnotify::Notification::new(&summary, Some(body.as_str()), Some(icon.as_str()));
    } else {
        n = libnotify::Notification::new(&summary, Some(body.as_str()), None);
    }

    // Show the notification
    if let Err(e) = n.show() {
        eprintln!("libnotify::Notification::show failed! {e}");
        ret = -2;
    }

    // We are done, deinit
    libnotify::uninit();

    ret
}
