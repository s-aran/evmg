pub mod environment_variable {
    use std::path::Path;

    pub trait EnvironmentVariable {
        fn list(&self) -> Vec<String>;

        fn get(&self, name: &String) -> Result<String, String>;
        fn set(&self, name: &String, value: &String) -> Result<(), String>;
        fn remove(&self, name: &String) -> Result<(), String>;

        fn get_path(&self) -> Result<Vec<String>, String>;
        fn append_path(&self, path: &Path) -> usize;
        fn insert_path(&self, path: &Path) -> usize;
        fn delete_path_by_index(&self, index: usize) -> usize;
        fn delete_path_by_value(&self, path: &Path) -> usize;
    }

    #[cfg(target_os = "windows")]
    pub mod env {
        use crate::envvar::environment_variable::EnvironmentVariable;

        use windows_sys::Win32::{Foundation::ERROR_SUCCESS, System::Registry::*};

        const ENVIRONMENT: &str = "Environment";
        const PATH: &str = "Path";
        const PATH_DELIMITER: char = ';';

        pub struct Environment;

        impl EnvironmentVariable for Environment {
            fn list(&self) -> Vec<String> {
                todo!()
            }

            fn get(&self, name: &String) -> Result<String, String> {
                let mut handle_key: HKEY = 0;

                if unsafe {
                    let mut wstr: Vec<u16> = ENVIRONMENT.encode_utf16().collect();
                    // teminal character
                    wstr.push(0x0000);
                    RegOpenKeyExW(
                        HKEY_CURRENT_USER,
                        wstr.as_ptr(),
                        0,
                        KEY_READ,
                        &mut handle_key,
                    )
                } != 0
                {
                    return Err("Cannot open user environment variable.".to_string());
                }

                let mut size: u32 = 0;
                let value_str: String;
                unsafe {
                    let mut wstr: Vec<u16> = name.encode_utf16().collect();
                    // teminal character
                    wstr.push(0x0000);

                    // get size
                    {
                        let r = RegQueryValueExW(
                            handle_key,
                            wstr.as_ptr(),
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            &mut size,
                        );

                        if r != ERROR_SUCCESS {
                            return Err(format!(
                                "Cannot open user {} registry. code: {} ({:#08X})",
                                ENVIRONMENT, r, r
                            ));
                        }
                    }

                    // get value
                    let mut value = vec![0u8; size as usize];
                    let r = RegQueryValueExW(
                        handle_key,
                        wstr.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        value.as_mut_ptr(),
                        &mut size,
                    );

                    if r != ERROR_SUCCESS {
                        return Err(format!(
                            "Cannot read uer {} registry. code: {} ({:#08X}",
                            ENVIRONMENT, r, r
                        ));
                    }

                    value_str = String::from_utf8(value).unwrap();
                }

                if handle_key != 0 {
                    unsafe { RegCloseKey(handle_key) };
                }

                Ok(value_str)
            }

            fn set(&self, name: &String, value: &String) -> Result<(), String> {
                let mut handle_key: HKEY = 0;

                if unsafe {
                    let mut wstr: Vec<u16> = ENVIRONMENT.encode_utf16().collect();
                    // terminal character
                    wstr.push(0x0000);
                    RegOpenKeyExW(
                        HKEY_CURRENT_USER,
                        wstr.as_ptr(),
                        0,
                        KEY_WRITE,
                        &mut handle_key,
                    )
                } != 0
                {
                    return Err("Cannto open user environment variable.".to_string());
                }

                unsafe {
                    let mut name_wstr: Vec<u16> = name.encode_utf16().collect();
                    // termianl character
                    name_wstr.push(0x0000);

                    let mut value_wstr: Vec<u16> = value.encode_utf16().collect();
                    // terminal character
                    value_wstr.push(0x0000);

                    // Vec<u16> -> Vec<u8>
                    let mut value_wstr8: Vec<u8> = Vec::new();
                    for e in &value_wstr {
                        value_wstr8.push((e & 0x00FF) as u8);
                        value_wstr8.push(((e & 0xFF00) >> 8) as u8);
                    }

                    RegSetValueExW(
                        handle_key,
                        name_wstr.as_ptr(),
                        0,
                        REG_EXPAND_SZ,
                        value_wstr8.as_ptr(),
                        value_wstr8.len() as u32,
                    );
                }

                if handle_key != 0 {
                    unsafe { RegCloseKey(handle_key) };
                }

                Ok(())
            }

            fn remove(&self, name: &String) -> Result<(), String> {
                let mut handle_key: HKEY = 0;

                if unsafe {
                    let mut wstr: Vec<u16> = ENVIRONMENT.encode_utf16().collect();
                    // terminal character
                    wstr.push(0x0000);
                    RegOpenKeyExW(
                        HKEY_CURRENT_USER,
                        wstr.as_ptr(),
                        0,
                        KEY_ALL_ACCESS,
                        &mut handle_key,
                    )
                } != 0
                {
                    return Err("Cannot open user environment variable.".to_string());
                }

                println!("{}", self.get(name).unwrap());

                unsafe {
                    let mut name_wstr: Vec<u16> = name.encode_utf16().collect();
                    // termianl character
                    name_wstr.push(0x0000);

                    let r = RegDeleteKeyW(handle_key, name_wstr.as_ptr());
                    if r != ERROR_SUCCESS {
                        return Err(format!(
                            "Cannot remove user environment variable. code: {}",
                            r
                        ));
                    }
                }

                if handle_key != 0 {
                    unsafe { RegCloseKey(handle_key) };
                }

                Ok(())
            }

            fn get_path(&self) -> Result<Vec<String>, String> {
                match self.get(&PATH.to_string()) {
                    Ok(e) => {
                        return Ok(e
                            .split(&PATH_DELIMITER.to_string())
                            .map(|s| s.trim().to_string())
                            .collect());
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            fn append_path(&self, path: &std::path::Path) -> usize {
                todo!()
            }

            fn insert_path(&self, path: &std::path::Path) -> usize {
                todo!()
            }

            fn delete_path_by_index(&self, index: usize) -> usize {
                todo!()
            }

            fn delete_path_by_value(&self, path: &std::path::Path) -> usize {
                todo!()
            }
        }

        impl Environment {
            pub fn new() -> Self {
                Environment {}
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub mod env {
        use crate::envvar::EnvironmentVariable;
    }
}
