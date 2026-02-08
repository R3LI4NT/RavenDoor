use std::process;
use windows::Win32::System::Threading::{
    CreateProcessW, PROCESS_CREATION_FLAGS, STARTUPINFOW, PROCESS_INFORMATION
};
use windows::core::PWSTR;
use windows::Win32::Foundation::BOOL;

pub fn hide_console() {
    unsafe {
        use windows::Win32::System::Console::{FreeConsole, AttachConsole, ATTACH_PARENT_PROCESS};
        
        if AttachConsole(ATTACH_PARENT_PROCESS).as_bool() {
            FreeConsole().ok();
        }
    }
}

pub fn get_windows_version() -> String {
    let output = Command::new("cmd")
        .args(&["/c", "ver"])
        .output()
        .unwrap_or_else(|_| process::Output {
            stdout: b"Unknown".to_vec(),
            stderr: Vec::new(),
            status: process::ExitStatus::default(),
        });
    
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn is_elevated() -> bool {
    use windows::Win32::Security::{OpenProcessToken, GetTokenInformation, TokenElevation, TOKEN_QUERY};
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::System::Threading::GetCurrentProcess;
    
    unsafe {
        let mut token_handle = HANDLE(0);
        let process_handle = GetCurrentProcess();
        
        if OpenProcessToken(
            process_handle,
            TOKEN_QUERY,
            &mut token_handle
        ).as_bool() {
            let mut elevation = TokenElevation::default();
            let mut size = 0;
            
            if GetTokenInformation(
                token_handle,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                std::mem::size_of::<TokenElevation>() as u32,
                &mut size
            ).as_bool() {
                return elevation.TokenIsElevated != 0;
            }
        }
    }
    
    false
}

pub fn encrypt_data(data: &[u8]) -> Vec<u8> {
    let key = b"RavenDoorKey2024";
    let mut result = Vec::with_capacity(data.len());
    
    for (i, &byte) in data.iter().enumerate() {
        result.push(byte ^ key[i % key.len()]);
    }
    
    result
}

pub fn decrypt_data(data: &[u8]) -> Vec<u8> {
    encrypt_data(data)
}