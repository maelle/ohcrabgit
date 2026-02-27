use std::sync::Mutex;
use zut::lang::Lang;

// Serialize env-var mutations across parallel tests.
static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn lang_codes() {
    assert_eq!(Lang::En.code(), "en");
    assert_eq!(Lang::Fr.code(), "fr");
    assert_eq!(Lang::Es.code(), "es");
}

#[test]
fn flag_takes_priority_over_env() {
    let _guard = ENV_MUTEX.lock().unwrap();
    unsafe { std::env::set_var("ZUT_LANG", "fr") };
    let lang = Lang::resolve(Some(Lang::Es));
    unsafe { std::env::remove_var("ZUT_LANG") };
    assert_eq!(lang, Lang::Es);
}

#[test]
fn env_var_fr() {
    let _guard = ENV_MUTEX.lock().unwrap();
    unsafe { std::env::set_var("ZUT_LANG", "fr") };
    let lang = Lang::resolve(None);
    unsafe { std::env::remove_var("ZUT_LANG") };
    assert_eq!(lang, Lang::Fr);
}

#[test]
fn env_var_strips_underscore_region() {
    let _guard = ENV_MUTEX.lock().unwrap();
    unsafe { std::env::set_var("ZUT_LANG", "fr_CA") };
    let lang = Lang::resolve(None);
    unsafe { std::env::remove_var("ZUT_LANG") };
    assert_eq!(lang, Lang::Fr);
}

#[test]
fn env_var_strips_hyphen_region() {
    let _guard = ENV_MUTEX.lock().unwrap();
    unsafe { std::env::set_var("ZUT_LANG", "es-MX") };
    let lang = Lang::resolve(None);
    unsafe { std::env::remove_var("ZUT_LANG") };
    assert_eq!(lang, Lang::Es);
}

#[test]
fn env_var_unknown_falls_back_to_en() {
    let _guard = ENV_MUTEX.lock().unwrap();
    unsafe { std::env::set_var("ZUT_LANG", "de") };
    let lang = Lang::resolve(None);
    unsafe { std::env::remove_var("ZUT_LANG") };
    assert_eq!(lang, Lang::En);
}

#[test]
fn no_env_var_falls_back_to_en_or_system() {
    let _guard = ENV_MUTEX.lock().unwrap();
    unsafe { std::env::remove_var("ZUT_LANG") };
    // Just verify it doesn't panic; result depends on system locale.
    let _ = Lang::resolve(None);
}
