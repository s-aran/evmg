pub mod environment_variable {
    use std::path::Path;

    pub trait EnvironmentVariable {
        fn list(&self) -> Vec<String>;

        fn get(&self, name: &String) -> Result<String, String>;
        fn set(&self, name: &String, value: &String) -> Result<(), String>;
        fn delete(&self, name: &String) -> Result<(), String>;

        fn get_list(&self, name: &String, delimiter: &String) -> Result<Vec<String>, String>;
        fn set_list(
            &self,
            name: &String,
            values: &Vec<String>,
            delimiter: &String,
        ) -> Result<(), String>;
        fn append_list(&self, name: &String, value: &String) -> Result<(), String>;
        fn insert_list(&self, name: &String, value: &String, to: usize) -> Result<(), String>;
        fn remove_list(&self, name: &String, by: usize) -> Result<(), String>;
        fn remove_list_from(&self, name: &String, value: &String) -> Result<(), String>;

        fn get_path(&self) -> Result<Vec<String>, String>;
        fn set_path(&self, paths: &Vec<String>) -> Result<(), String>;
        fn append_path(&self, path: &Path) -> Result<(), String>;
        fn insert_path(&self, path: &Path, to: usize) -> Result<(), String>;
        fn remove_path(&self, by: usize) -> Result<(), String>;
        fn remove_path_from(&self, path: &Path) -> Result<(), String>;
    }

    #[cfg(target_os = "windows")]
    pub mod env {
        use std::path::Path;

        use crate::envvar::environment_variable::EnvironmentVariable;

        use windows_sys::Win32::{Foundation::ERROR_SUCCESS, System::Registry::*};

        const ENVIRONMENT: &str = "Environment";
        const PATH: &str = "Path";
        const PATH_DELIMITER: &str = ";";

        pub struct Environment;

        impl Environment {
            pub fn new() -> Self {
                Environment {}
            }

            fn string_to_u16vec(s: &String) -> Vec<u16> {
                let mut ws: Vec<u16> = s.encode_utf16().collect();
                // terminal character
                ws.push(0x0000);

                ws
            }

            fn string_to_u8vec(s: &String) -> Vec<u8> {
                let u16vec = Self::string_to_u16vec(s);
                let mut u8vec: Vec<u8> = Vec::new();
                for e in &u16vec {
                    u8vec.push((e & 0x00FF) as u8);
                    u8vec.push(((e & 0xFF00) >> 8) as u8);
                }

                u8vec
            }

            fn open_registry(
                hkey: isize,
                subkey: &String,
                samdesired: u32,
            ) -> Result<HKEY, String> {
                let mut handle_key: HKEY = 0;
                let subkey_u16vec = Self::string_to_u16vec(subkey);
                let lpsubkey = subkey_u16vec.as_ptr();

                let r = unsafe { RegOpenKeyExW(hkey, lpsubkey, 0, samdesired, &mut handle_key) };
                match r {
                    ERROR_SUCCESS => Ok(handle_key),
                    _ => Err(format!(
                        "Cannot open user environment variable. code: {}",
                        r
                    )),
                }
            }

            fn close_registry(handle: HKEY) -> Result<(), String> {
                let r = unsafe { RegCloseKey(handle) };
                match r {
                    ERROR_SUCCESS => Ok(()),
                    _ => Err(format!("Cannot close registry handle. code: {}", r)),
                }
            }

            fn read_registry(hkey: HKEY, valuename: &String) -> Result<String, String> {
                let value_u16vec = Self::string_to_u16vec(valuename);
                let mut size: u32 = 0;

                // get actual data size
                let calc_size_result = unsafe {
                    RegQueryValueExW(
                        hkey,
                        value_u16vec.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        &mut size,
                    )
                };

                if calc_size_result != ERROR_SUCCESS {
                    return Err(format!(
                        "Cannot read user registry for get data size. code: {}",
                        calc_size_result
                    ));
                }

                // get data
                let mut data = vec![0u8; size as usize];
                let r = unsafe {
                    RegQueryValueExW(
                        hkey,
                        value_u16vec.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        data.as_mut_ptr(),
                        &mut size,
                    )
                };

                match r {
                    ERROR_SUCCESS => Ok(String::from_utf8_lossy(&data).to_string()),
                    _ => Err(format!("Cannot read user registry, code: {}", r)),
                }
            }

            fn write_registry(hkey: HKEY, valuename: &String, data: &String) -> Result<(), String> {
                let value_u16vec = Self::string_to_u16vec(valuename);
                let data_u8vec = Self::string_to_u8vec(data);
                let r = unsafe {
                    RegSetValueExW(
                        hkey,
                        value_u16vec.as_ptr(),
                        0,
                        REG_EXPAND_SZ,
                        data_u8vec.as_ptr(),
                        data_u8vec.len() as u32,
                    )
                };

                match r {
                    ERROR_SUCCESS => Ok(()),
                    _ => Err(format!("Cannot write user registry. code: {}", r)),
                }
            }

            fn delete_registry(hkey: HKEY, valuename: &String) -> Result<(), String> {
                let value_u16vec = Self::string_to_u16vec(valuename);
                let r = unsafe { RegDeleteValueW(hkey, value_u16vec.as_ptr()) };

                match r {
                    ERROR_SUCCESS => Ok(()),
                    _ => Err(format!("Cannot delete user registry value. code: {}", r)),
                }
            }
        }

        impl EnvironmentVariable for Environment {
            fn list(&self) -> Vec<String> {
                todo!()
            }

            fn get(&self, name: &String) -> Result<String, String> {
                let open_result =
                    Self::open_registry(HKEY_CURRENT_USER, &ENVIRONMENT.to_string(), KEY_READ);

                if open_result.is_err() {
                    return Err(open_result.unwrap_err());
                }

                let handle_key: HKEY = open_result.unwrap();

                let read_result = Self::read_registry(handle_key, name);

                let close_result = Self::close_registry(handle_key);
                if close_result.is_err() {
                    return Err(close_result.unwrap_err());
                }

                if read_result.is_err() {
                    return Err(read_result.unwrap_err());
                }

                Ok(read_result.unwrap())
            }

            fn set(&self, name: &String, value: &String) -> Result<(), String> {
                let open_result =
                    Self::open_registry(HKEY_CURRENT_USER, &ENVIRONMENT.to_string(), KEY_WRITE);

                if open_result.is_err() {
                    return Err(open_result.unwrap_err());
                }

                let handle_key: HKEY = open_result.unwrap();

                let write_result = Self::write_registry(handle_key, &name, &value);

                let close_result = Self::close_registry(handle_key);
                if close_result.is_err() {
                    return Err(close_result.unwrap_err());
                }

                if write_result.is_err() {
                    return Err(write_result.unwrap_err());
                }

                Ok(())
            }

            fn delete(&self, name: &String) -> Result<(), String> {
                let open_result =
                    Self::open_registry(HKEY_CURRENT_USER, &ENVIRONMENT.to_string(), KEY_WRITE);

                if open_result.is_err() {
                    return Err(open_result.unwrap_err());
                }

                let handle_key: HKEY = open_result.unwrap();

                let delete_result = Self::delete_registry(handle_key, name);

                let close_result = Self::close_registry(handle_key);
                if close_result.is_err() {
                    return Err(close_result.unwrap_err());
                }

                if delete_result.is_err() {
                    return Err(delete_result.unwrap_err());
                }

                Ok(())
            }

            fn get_list(&self, name: &String, delimiter: &String) -> Result<Vec<String>, String> {
                match self.get(name) {
                    Ok(e) => {
                        return Ok(e.split(delimiter).map(|s| s.trim().to_string()).collect());
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            fn set_list(
                &self,
                name: &String,
                values: &Vec<String>,
                delimiter: &String,
            ) -> Result<(), String> {
                let s = values.join(delimiter);
                println!("{}", s);
                todo!()
            }

            fn append_list(&self, name: &String, value: &String) -> Result<(), String> {
                todo!()
            }

            fn insert_list(&self, name: &String, value: &String, to: usize) -> Result<(), String> {
                todo!()
            }

            fn remove_list(&self, name: &String, by: usize) -> Result<(), String> {
                todo!()
            }

            fn remove_list_from(&self, name: &String, value: &String) -> Result<(), String> {
                todo!()
            }

            fn get_path(&self) -> Result<Vec<String>, String> {
                self.get_list(&PATH.to_string(), &PATH_DELIMITER.to_string())
            }

            fn set_path(&self, paths: &Vec<String>) -> Result<(), String> {
                todo!()
            }

            fn append_path(&self, path: &Path) -> Result<(), String> {
                todo!()
            }

            fn insert_path(&self, path: &Path, to: usize) -> Result<(), String> {
                todo!()
            }

            fn remove_path(&self, by: usize) -> Result<(), String> {
                todo!()
            }

            fn remove_path_from(&self, path: &Path) -> Result<(), String> {
                todo!()
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub mod env {
        use crate::envvar::EnvironmentVariable;
    }
}
