#[derive(Clone)]
pub struct Response<'a> {
    pub status_code: u16,
    pub status_message: &'a str,
    pub date: &'a str,
    pub body: Vec<u8>,
    pub size: usize
}
