use std::{
    char::{decode_utf16, REPLACEMENT_CHARACTER},
    iter,
    mem::size_of,
    sync::LazyLock,
};

use winapi::{
    shared::minwindef::MAX_PATH,
    um::commdlg::{OFN_FILEMUSTEXIST, OFN_OVERWRITEPROMPT, OPENFILENAMEW},
};

pub static FILTER_JSON: LazyLock<Vec<u16>> = LazyLock::new(|| {
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
});

pub fn get_open_file_name(filters: Option<&[u16]>) -> Option<String> {
    let mut buf = vec![0u16; MAX_PATH];

    let mut ofn: OPENFILENAMEW = OPENFILENAMEW {
        lStructSize: size_of::<winapi::um::commdlg::OPENFILENAMEW>() as u32,
        nMaxFile: buf.len() as u32,
        Flags: OFN_FILEMUSTEXIST,
        ..Default::default()
    };

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

    let mut ofn: OPENFILENAMEW = OPENFILENAMEW {
        lStructSize: size_of::<winapi::um::commdlg::OPENFILENAMEW>() as u32,
        nMaxFile: buf.len() as u32,
        Flags: OFN_OVERWRITEPROMPT,
        ..Default::default()
    };

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
