//! Read-only evidence for this app ID only. Snapshot BEFORE clicking Send test reminder.
//! notification_probe --snapshot before.json
//! notification_probe --since before.json
#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use windows::{core::HSTRING, UI::Notifications::ToastNotificationManager};
    let args: Vec<String> = std::env::args().skip(1).collect();
    let before: Option<Vec<String>> = match args.as_slice() {
        [] => None,
        [flag, path] if flag == "--since" => {
            Some(serde_json::from_str(&std::fs::read_to_string(path)?)?)
        }
        [flag, _] if flag == "--snapshot" => None,
        _ => return Err("Usage: notification_probe [--snapshot FILE | --since FILE]".into()),
    };
    let history = ToastNotificationManager::History()?
        .GetHistoryWithId(&HSTRING::from("com.pausepal.desktop"))?;
    let count = history.Size()?;
    let mut entries = Vec::new();
    let mut unreadable = 0_u32;
    for index in 0..count {
        // Old Windows history entries can have unreadable XML. Preserve them;
        // they cannot prove a fresh notification, but must not hide readable ones.
        let notification = match history.GetAt(index) {
            Ok(notification) => notification,
            Err(_) => {
                unreadable += 1;
                continue;
            }
        };
        let xml = match notification.Content().and_then(|xml| xml.GetXml()) {
            Ok(xml) => xml.to_string_lossy(),
            Err(_) => {
                unreadable += 1;
                continue;
            }
        };
        let identity = format!(
            "{}|{}|{}",
            notification.Tag()?.to_string_lossy(),
            notification.Group()?.to_string_lossy(),
            xml
        );
        entries.push(identity);
    }
    let (found, has_actions) = fresh_test(&entries, before.as_deref());
    if let [flag, path] = args.as_slice() {
        if flag == "--snapshot" {
            std::fs::write(path, serde_json::to_vec(&entries)?)?;
            println!("{{\"appId\":\"com.pausepal.desktop\",\"snapshotCount\":{count},\"unreadableHistoryCount\":{unreadable}}}");
            return Ok(());
        }
    }
    println!("{{\"appId\":\"com.pausepal.desktop\",\"historyCount\":{count},\"unreadableHistoryCount\":{unreadable},\"freshCheck\":{},\"testReminderFound\":{found},\"testHasActionButtons\":{has_actions}}}", before.is_some());
    if !found || has_actions {
        std::process::exit(2);
    }
    Ok(())
}

#[cfg(any(windows, test))]
fn fresh_test(entries: &[String], before: Option<&[String]>) -> (bool, bool) {
    // Windows may give repeated notifications identical XML and empty tags.
    // Compare multisets: each old occurrence consumes exactly one current row.
    let mut remaining = before.unwrap_or_default().to_vec();
    let (mut found, mut actions) = (false, false);
    for entry in entries {
        if let Some(index) = remaining.iter().position(|old| old == entry) {
            remaining.swap_remove(index);
            continue;
        }
        if entry.contains("Restnaa test reminder") {
            found = true;
            actions |= entry.contains("<actions");
        }
    }
    (found, actions)
}

#[cfg(test)]
mod tests {
    use super::fresh_test;
    #[test]
    fn unchanged_snapshot_fails_but_an_additional_identical_toast_is_fresh() {
        let old = vec!["||<toast>Restnaa test reminder</toast>".to_owned()];
        assert_eq!(fresh_test(&old, Some(&old)), (false, false));
        let mut new = old.clone();
        new.push(old[0].clone());
        assert_eq!(fresh_test(&new, Some(&old)), (true, false));
    }
    #[test]
    fn non_test_entries_do_not_pass_and_fresh_action_markup_is_detected() {
        assert_eq!(
            fresh_test(
                &["||<toast>Time for an eye break</toast>".into()],
                Some(&[])
            ),
            (false, false)
        );
        assert_eq!(
            fresh_test(
                &["||<toast>Restnaa test reminder<actions/></toast>".into()],
                Some(&[])
            ),
            (true, true)
        );
    }
}
#[cfg(not(windows))]
fn main() {
    eprintln!("This read-only notification probe requires Windows.");
    std::process::exit(2);
}
