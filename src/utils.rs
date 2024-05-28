#[derive(Debug)]
pub enum Status {
    Untracked,      // ??
    Staged,         // A
    Modified,       // M
    StagedModified, // AM
    Unknown,
}

pub fn code_to_enum(code: &str) -> Status {
    match code {
        "??" => Status::Untracked,
        "A" => Status::Staged,
        "M" => Status::Modified,
        "AM" => Status::StagedModified,
        _ => Status::Unknown,
    }
}
