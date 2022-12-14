// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JObject};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jdouble};

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_Main_negDistance(env: JNIEnv,
                                             _class: JClass,
                                             _complex: JObject)
                                             -> jdouble {
    let re = env.get_field(_complex, "re", "D").unwrap().d().unwrap();
    let im = env.get_field(_complex, "im", "D").unwrap().d().unwrap();
    println!("Get \"re\" and \"im\" inside rust, \"re\" = {}, \"im\" = {}", re, im);
    let java_dist = env.call_method(_complex, "Distance", "()D", &[]).unwrap().d().unwrap();
    let neg_dist = -java_dist;
    neg_dist
}
