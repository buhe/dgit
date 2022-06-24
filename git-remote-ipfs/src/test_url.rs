use log::info;

pub fn print_with_url(url: impl AsRef<str>) {
    let url = url.as_ref();
    info!("qr is {}", url);
}