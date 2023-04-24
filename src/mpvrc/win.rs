use std::{
    char::{decode_utf16, REPLACEMENT_CHARACTER},
    mem::size_of,
};

use winapi::{shared::minwindef::MAX_PATH, um::commdlg::OPENFILENAMEW};

pub fn get_open_file_name() -> Option<String> {
    let mut buf = vec![0u16; MAX_PATH];

    let mut ofn: OPENFILENAMEW = Default::default();

    ofn.lStructSize = size_of::<winapi::um::commdlg::OPENFILENAMEW>() as u32;
    ofn.nMaxFile = buf.len() as u32;

    unsafe {
        ofn.lpstrFile = buf.as_mut_ptr();
        if winapi::um::commdlg::GetOpenFileNameW(&mut ofn) == 0 {
            return None;
        }
    }

    Some(from_utf16_buf(buf))
}

pub fn get_save_file_name() -> Option<String> {
    let mut buf = vec![0u16; MAX_PATH];

    let mut ofn: OPENFILENAMEW = Default::default();

    ofn.lStructSize = size_of::<winapi::um::commdlg::OPENFILENAMEW>() as u32;
    ofn.nMaxFile = buf.len() as u32;

    unsafe {
        ofn.lpstrFile = buf.as_mut_ptr();
        if winapi::um::commdlg::GetSaveFileNameW(&mut ofn) == 0 {
            return None;
        }
    }

    Some(from_utf16_buf(buf))
}

fn from_utf16_buf<I>(buf: I) -> String
where
    I: IntoIterator<Item = u16>,
{
    decode_utf16(buf.into_iter().take_while(|&x| x != 0))
        .map(|c| c.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}
