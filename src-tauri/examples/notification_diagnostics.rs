//! Read-only Windows diagnostics scoped to Restna; never clears OS history.
#[cfg(windows)]
fn main() {
    use windows::{core::HSTRING, UI::Notifications::ToastNotificationManager};
    let id = HSTRING::from("com.pausepal.desktop");
    match ToastNotificationManager::CreateToastNotifierWithId(&id).and_then(|n| n.Setting()) {
        Ok(setting) => println!("permission={setting:?}"),
        Err(error) => println!("permission_error={error}"),
    }
    let history = match ToastNotificationManager::History().and_then(|h| h.GetHistoryWithId(&id)) {
        Ok(history) => history,
        Err(error) => {
            println!("history_query_error={error}");
            return;
        }
    };
    let count = match history.Size() {
        Ok(count) => count,
        Err(error) => {
            println!("history_size_error={error}");
            return;
        }
    };
    println!("history_count={count}");
    let mut readable = 0;
    let mut invalid = 0;
    for index in 0..count {
        match history
            .GetAt(index)
            .and_then(|n| n.Content())
            .and_then(|xml| xml.GetXml())
        {
            Ok(xml) => {
                readable += 1;
                println!("entry={index} xml_chars={}", xml.len());
            }
            Err(error) => {
                invalid += 1;
                println!("entry={index} content_error={error}");
            }
        }
    }
    println!("readable={readable} invalid={invalid}");
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows only");
}
