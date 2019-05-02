#[derive(Debug)]
pub enum Action {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match &self {
            Action::Typing => "typing".into(),
            Action::UploadPhoto => "upload_photo".into(),
            Action::RecordVideo => "record_video".into(),
            Action::UploadVideo => "upload_video".into(),
            Action::RecordAudio => "record_audio".into(),
            Action::UploadAudio => "upload_audio".into(),
            Action::UploadDocument => "upload_document".into(),
            Action::FindLocation => "find_location".into(),
            Action::RecordVideoNote => "record_video_note".into(),
            Action::UploadVideoNote => "upload_video_note".into()
        }
    }
}