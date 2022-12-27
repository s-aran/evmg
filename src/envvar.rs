pub mod environment_variable {

    use std::path::Path;

    pub trait EnvironmentVariable {
        fn list(&self) -> Result<Vec<(String, String)>, String>;

        fn get(&self, name: &String) -> Result<String, String>;
        fn set(&mut self, name: &String, value: &String) -> Result<(), String>;
        fn delete(&mut self, name: &String) -> Result<(), String>;

        fn get_list(&self, name: &String, delimiter: &String) -> Result<Vec<String>, String> {
            match self.get(name) {
                Ok(e) => {
                    let chars = &e.chars().collect::<Vec<char>>();
                    let removed_terminal = &chars[0..e.len() - 1].iter().collect::<String>();
                    Ok(removed_terminal
                        .split(delimiter)
                        .map(|s| s.to_string())
                        .collect())
                }
                Err(e) => Err(e),
            }
        }

        fn set_list(
            &mut self,
            name: &String,
            values: &Vec<String>,
            delimiter: &String,
        ) -> Result<(), String> {
            let s = values.join(delimiter);
            self.set(name, &s)
        }

        fn append_list(
            &mut self,
            name: &String,
            value: &String,
            delimiter: &String,
        ) -> Result<(), String> {
            match self.get_list(name, delimiter) {
                Ok(l) => {
                    let mut ll = l;
                    ll.push(value.to_string());
                    self.set_list(name, &ll, delimiter)
                }
                Err(s) => Err(s),
            }
        }

        fn insert_list(
            &mut self,
            name: &String,
            value: &String,
            to: usize,
            delimiter: &String,
        ) -> Result<(), String> {
            match self.get_list(name, delimiter) {
                Ok(l) => {
                    let mut ll = l;
                    ll.insert(to, value.to_string());
                    self.set_list(name, &ll, delimiter)
                }
                Err(s) => Err(s),
            }
        }

        fn remove_list(
            &mut self,
            name: &String,
            from: usize,
            delimiter: &String,
        ) -> Result<(), String> {
            match self.get_list(name, delimiter) {
                Ok(l) => {
                    let mut ll = l;
                    ll.remove(from);
                    self.set_list(name, &ll, delimiter)
                }
                Err(s) => Err(s),
            }
        }

        fn remove_list_from(
            &mut self,
            name: &String,
            value: &String,
            delimiter: &String,
        ) -> Result<(), String> {
            match self.get_list(name, delimiter) {
                Ok(l) => {
                    let ll = l
                        .iter()
                        .filter(|&e| e != value)
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>();
                    self.set_list(name, &ll, delimiter)
                }
                Err(s) => Err(s),
            }
        }

        fn get_path(&self) -> Result<Vec<String>, String>;
        fn set_path(&mut self, paths: &Vec<String>) -> Result<(), String>;
        fn append_path(&mut self, path: &Path) -> Result<(), String>;
        fn insert_path(&mut self, path: &Path, to: usize) -> Result<(), String>;
        fn remove_path(&mut self, by: usize) -> Result<(), String>;
        fn remove_path_from(&mut self, path: &Path) -> Result<(), String>;
    }

    #[cfg(target_os = "windows")]
    pub mod env {
        use std::path::Path;

        use crate::envvar::environment_variable::EnvironmentVariable;

        use windows_sys::Win32::{
            Foundation::{ERROR_NO_MORE_ITEMS, ERROR_SUCCESS, MAX_PATH},
            System::Registry::*,
        };

        const ENVIRONMENT: &str = "Environment";
        pub const PATH: &str = "Path";
        pub const PATH_DELIMITER: &str = ";";

        const IGNORE_KEYS: [&str; 1] = ["__PSLockDownPolicy"];

        pub struct Environment {}

        impl Environment {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl EnvironmentVariable for Environment {
            fn list(&self) -> Result<Vec<(String, String)>, String> {
                let vars = env::vars();
                let env_list = vars.collect::<Vec<(String, String)>>();
                let ignored = env_list
                    .iter()
                    .filter(|(k, _)| !IGNORE_KEYS.contains(&(*k).as_str()));
                Ok(ignored.cloned().collect::<Vec<(String, String)>>())
            }

            fn get(&self, name: &String) -> Result<String, String> {
                todo!()
            }

            fn set(&mut self, name: &String, value: &String) -> Result<(), String> {
                todo!()
            }

            fn delete(&mut self, name: &String) -> Result<(), String> {
                todo!()
            }

            fn get_path(&self) -> Result<Vec<String>, String> {
                todo!()
            }

            fn set_path(&mut self, paths: &Vec<String>) -> Result<(), String> {
                todo!()
            }

            fn append_path(&mut self, path: &Path) -> Result<(), String> {
                todo!()
            }

            fn insert_path(&mut self, path: &Path, to: usize) -> Result<(), String> {
                todo!()
            }

            fn remove_path(&mut self, by: usize) -> Result<(), String> {
                todo!()
            }

            fn remove_path_from(&mut self, path: &Path) -> Result<(), String> {
                todo!()
            }
        }

        pub struct EnvironmentRegistry;

        impl EnvironmentRegistry {
            pub fn new() -> Self {
                Self {}
            }

            fn string_to_u16vec(s: &String) -> Vec<u16> {
                let mut ws: Vec<u16> = s.encode_utf16().collect();
                // terminal character
                ws.push(0x0000);

                ws
            }

            fn string_to_u8vec(s: &String) -> Vec<u8> {
                // let cc = s.chars().map(|e| e as u8).collect::<Vec<u8>>();
                let u16vec: Vec<u16> = Self::string_to_u16vec(s);
                let mut u8vec: Vec<u8> = Vec::with_capacity(u16vec.len() << 1);
                for e in u16vec.iter() {
                    u8vec.push((e & 0x00FF) as u8);
                    u8vec.push(((e & 0xFF00) >> 8) as u8);
                }

                u8vec
            }

            fn u8vec_to_string(data: &Vec<u8>) -> String {
                let mut d = vec![0u16; 0];
                let mut t = 0;

                for (i, e) in data.iter().enumerate() {
                    if (i & 0x01) == 0 {
                        t = *e as u16;
                    } else {
                        t |= (*e as u16) << 8;
                        d.push(t);
                    }
                }

                if (data.len() & 0x01) == 1 {
                    d.push(t);
                }

                String::from_utf16_lossy(&d).to_string()
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
                    ERROR_SUCCESS => Ok(Self::u8vec_to_string(&data)),
                    _ => Err(format!("Cannot read user registry. code: {}", r)),
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

            fn get_registry_value_by_index(
                hkey: HKEY,
                index: u32,
            ) -> Result<Option<(String, String)>, String> {
                let mut value_u16vec = vec![0u16; MAX_PATH as usize];
                let mut value_size: u32 = MAX_PATH;

                let mut data_size: u32 = 0;

                // get actual data size
                let calc_size_result = unsafe {
                    RegEnumValueW(
                        hkey,
                        index,
                        value_u16vec.as_mut_ptr(),
                        &mut value_size,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        &mut data_size,
                    )
                };

                if calc_size_result != ERROR_SUCCESS && calc_size_result != ERROR_NO_MORE_ITEMS {
                    return Err(format!(
                        "Cannot read user registry. code: {}",
                        calc_size_result
                    ));
                }

                // get data
                value_size += std::mem::size_of::<u16>() as u32; // because value_size is not including terminating null character
                let mut data = vec![0u8; data_size as usize];
                let r = unsafe {
                    RegEnumValueW(
                        hkey,
                        index,
                        value_u16vec.as_mut_ptr(),
                        &mut value_size,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        data.as_mut_ptr(),
                        &mut data_size,
                    )
                };

                match r {
                    ERROR_SUCCESS => {
                        value_u16vec.truncate(value_size as usize);
                        data.truncate(data_size as usize - std::mem::size_of::<u16>());
                        Ok(Some((
                            String::from_utf16_lossy(&value_u16vec),
                            Self::u8vec_to_string(&data),
                        )))
                    }
                    ERROR_NO_MORE_ITEMS => Ok(None),
                    _ => Err(format!("Cannot read user registry. code: {}", r)),
                }
            }
        }

        impl EnvironmentVariable for EnvironmentRegistry {
            fn list(&self) -> Result<Vec<(String, String)>, String> {
                let mut result: Vec<(String, String)> = Vec::new();

                let open_result =
                    Self::open_registry(HKEY_CURRENT_USER, &ENVIRONMENT.to_string(), KEY_READ);
                if open_result.is_err() {
                    return Err(open_result.unwrap_err());
                }

                let handle_key: HKEY = open_result.unwrap();

                let mut i = 0;
                loop {
                    let get_result = Self::get_registry_value_by_index(handle_key, i);
                    if get_result.is_err() {
                        let _ = Self::close_registry(handle_key);
                        return Err(get_result.unwrap_err());
                    }

                    match get_result.unwrap() {
                        Some((v, d)) => result.push((v.as_str().to_string(), d)),
                        None => break,
                    }

                    i += 1;
                }

                let close_result = Self::close_registry(handle_key);
                if close_result.is_err() {
                    return Err(close_result.unwrap_err());
                }

                Ok(result)
            }

            fn get(&self, name: &String) -> Result<String, String> {
                let open_result =
                    Self::open_registry(HKEY_CURRENT_USER, &ENVIRONMENT.to_string(), KEY_READ);
                if open_result.is_err() {
                    return Err(open_result.unwrap_err());
                }

                let handle_key: HKEY = open_result.unwrap();

                let read_result = Self::read_registry(handle_key, name);
                if read_result.is_err() {
                    let _ = Self::close_registry(handle_key);
                    return Err(read_result.unwrap_err());
                }

                let close_result = Self::close_registry(handle_key);
                if close_result.is_err() {
                    return Err(close_result.unwrap_err());
                }

                Ok(read_result.unwrap())
            }

            fn set(&mut self, name: &String, value: &String) -> Result<(), String> {
                let open_result =
                    Self::open_registry(HKEY_CURRENT_USER, &ENVIRONMENT.to_string(), KEY_WRITE);

                if open_result.is_err() {
                    return Err(open_result.unwrap_err());
                }
                let handle_key: HKEY = open_result.unwrap();

                let write_result = Self::write_registry(handle_key, &name, &value);
                if write_result.is_err() {
                    let _ = Self::close_registry(handle_key);
                    return Err(write_result.unwrap_err());
                }

                let close_result = Self::close_registry(handle_key);
                if close_result.is_err() {
                    return Err(close_result.unwrap_err());
                }

                Ok(())
            }

            fn delete(&mut self, name: &String) -> Result<(), String> {
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

            fn get_path(&self) -> Result<Vec<String>, String> {
                self.get_list(&PATH.to_string(), &PATH_DELIMITER.to_string())
            }

            fn set_path(&mut self, paths: &Vec<String>) -> Result<(), String> {
                self.set_list(&PATH.to_string(), paths, &PATH_DELIMITER.to_string())
            }

            fn append_path(&mut self, path: &Path) -> Result<(), String> {
                self.append_list(
                    &PATH.to_string(),
                    &path.to_string_lossy().to_string(),
                    &PATH_DELIMITER.to_string(),
                )
            }

            fn insert_path(&mut self, path: &Path, to: usize) -> Result<(), String> {
                self.insert_list(
                    &PATH.to_string(),
                    &path.to_string_lossy().to_string(),
                    to,
                    &PATH_DELIMITER.to_string(),
                )
            }

            fn remove_path(&mut self, from: usize) -> Result<(), String> {
                self.remove_list(&PATH.to_string(), from, &PATH_DELIMITER.to_string())
            }

            fn remove_path_from(&mut self, path: &Path) -> Result<(), String> {
                self.remove_list_from(
                    &PATH.to_string(),
                    &path.to_string_lossy().to_string(),
                    &PATH_DELIMITER.to_string(),
                )
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub mod env {
        use libc::{getenv, setenv};
        use std::env;
        use std::ffi::CStr;
        use std::os::raw::c_char;
        use std::path::Path;

        use crate::envvar::environment_variable::EnvironmentVariable;
        use crate::shellrc::shellrc::{ShellRunCommandFile, ShellRunCommandFileData};

        pub const PATH: &str = "PATH";
        pub const PATH_DELIMITER: &str = ":";

        const IGNORE_KEYS: [&str; 3] = ["_", "PWD", "SHLVL"];

        pub struct Environment {
            shellrc: Option<ShellRunCommandFileData>,
        }

        impl Environment {
            pub fn new() -> Self {
                Self { shellrc: None }
            }

            pub fn init_shell(&mut self, shell: &String) {
                if self.shellrc.is_none() {
                    self.shellrc = Some({
                        let mut d = ShellRunCommandFileData::new(shell);
                        // set current environment variables
                        for (k, v) in self.list().unwrap().iter() {
                            d.add(k, v);
                        }
                        d
                    });

                    return;
                }
            }

            pub fn write_rc(&self, filepath: &Path) -> Result<(), String> {
                self.shellrc.as_ref().unwrap().write(filepath)
            }

            fn string_to_i8vec(s: &String) -> Vec<i8> {
                let mut ws: Vec<i8> = s.chars().map(|e| e as i8).collect();
                // terminal character
                ws.push(0x00);

                ws
            }

            fn c_char_to_str<'a>(lpsz: *const c_char) -> Result<&'a str, String> {
                match unsafe { CStr::from_ptr(lpsz) }.to_str() {
                    Ok(s) => Ok(s),
                    Err(e) => Err(e.to_string()),
                }
            }

            fn write(&self, output_path: &String) {
                let s = match &self.shellrc {
                    Some(e) => e,
                    None => panic!("shell not initialized"),
                };

                match s.write(&Path::new(&output_path)) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                }
            }
        }

        impl EnvironmentVariable for Environment {
            fn list(&self) -> Result<Vec<(String, String)>, String> {
                let vars = env::vars();
                let env_list = vars.collect::<Vec<(String, String)>>();
                let ignored = env_list
                    .iter()
                    .filter(|(k, _)| !IGNORE_KEYS.contains(&(*k).as_str()));
                Ok(ignored.cloned().collect::<Vec<(String, String)>>())
            }

            fn get(&self, name: &String) -> Result<String, String> {
                let name_i8vec = Self::string_to_i8vec(name);
                let v: *mut c_char = unsafe { getenv(name_i8vec.as_ptr()) };
                match Self::c_char_to_str(v) {
                    Ok(s) => Ok(s.to_string()),
                    Err(e) => Err(e),
                }
            }

            fn set(&mut self, name: &String, value: &String) -> Result<(), String> {
                (&mut self.shellrc).as_mut().unwrap().set(name, value);
                Ok(())
            }

            fn delete(&mut self, name: &String) -> Result<(), String> {
                (&mut self.shellrc).as_mut().unwrap().delete(name)
            }

            fn get_path(&self) -> Result<Vec<String>, String> {
                self.get_list(&PATH.to_string(), &PATH_DELIMITER.to_string())
            }

            fn set_path(&mut self, paths: &Vec<String>) -> Result<(), String> {
                self.set_list(&PATH.to_string(), paths, &PATH_DELIMITER.to_string())
            }

            fn append_path(&mut self, path: &Path) -> Result<(), String> {
                self.append_list(
                    &PATH.to_string(),
                    &path.to_string_lossy().to_string(),
                    &PATH_DELIMITER.to_string(),
                )
            }

            fn insert_path(&mut self, path: &Path, to: usize) -> Result<(), String> {
                self.insert_list(
                    &PATH.to_string(),
                    &path.to_string_lossy().to_string(),
                    to,
                    &PATH_DELIMITER.to_string(),
                )
            }

            fn remove_path(&mut self, from: usize) -> Result<(), String> {
                self.remove_list(&PATH.to_string(), from, &PATH_DELIMITER.to_string())
            }

            fn remove_path_from(&mut self, path: &Path) -> Result<(), String> {
                self.remove_list_from(
                    &PATH.to_string(),
                    &path.to_string_lossy().to_string(),
                    &PATH_DELIMITER.to_string(),
                )
            }
        }
    }
}
