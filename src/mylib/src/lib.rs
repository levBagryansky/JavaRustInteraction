// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_HelloWorld_concat(env: JNIEnv,
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.
                                             _class: JClass,
                                             word1: JString,
                                             word2: JString)
                                             -> jstring {
    let word1: String =
        env.get_string(word1).expect("Couldn't get java string!").into();

    let word2: String =
        env.get_string(word2).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("{} {}", word1, word2))
        .expect("Couldn't create java string!");

    print!("Hello from rust code\n");
    // Finally, extract the raw pointer to return.
    output.into_raw()
}
