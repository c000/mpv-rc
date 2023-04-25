use std::{
    char::{decode_utf16, REPLACEMENT_CHARACTER},
    iter,
    mem::size_of,
};

use lazy_static::lazy_static;
use winapi::{
    shared::minwindef::MAX_PATH,
    um::commdlg::{OFN_FILEMUSTEXIST, OFN_OVERWRITEPROMPT, OPENFILENAMEW},
};

lazy_static! {
    pub static ref FILTER_JSON: Vec<u16> = {
        iter::empty()
            .chain(
                "JSON"
                    .encode_utf16()
                    .chain(iter::once(0))
                    .chain("*.json".encode_utf16())
                    .chain(iter::once(0)),
            )
            .chain(
                "All"
                    .encode_utf16()
                    .chain(iter::once(0))
                    .chain("*.*".encode_utf16())
                    .chain(iter::once(0)),
            )
            .chain(iter::once(0))
            .collect()
    };
}

pub fn get_open_file_name(filters: Option<&[u16]>) -> Option<String> {
    let mut buf = vec![0u16; MAX_PATH];

    let mut ofn: OPENFILENAMEW = Default::default();

    ofn.lStructSize = size_of::<winapi::um::commdlg::OPENFILENAMEW>() as u32;
    ofn.nMaxFile = buf.len() as u32;
    ofn.Flags = OFN_FILEMUSTEXIST;
    if let Some(f) = filters {
        ofn.lpstrFilter = f.as_ptr();
    }

    unsafe {
        ofn.lpstrFile = buf.as_mut_ptr();
        if winapi::um::commdlg::GetOpenFileNameW(&mut ofn) == 0 {
            return None;
        }
    }

    Some(from_utf16_buf(buf))
}

pub fn get_save_file_name(filters: Option<&[u16]>) -> Option<String> {
    let mut buf = vec![0u16; MAX_PATH];

    let mut ofn: OPENFILENAMEW = Default::default();

    ofn.lStructSize = size_of::<winapi::um::commdlg::OPENFILENAMEW>() as u32;
    ofn.nMaxFile = buf.len() as u32;
    ofn.Flags = OFN_OVERWRITEPROMPT;
    if let Some(f) = filters {
        ofn.lpstrFilter = f.as_ptr();
    }

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
