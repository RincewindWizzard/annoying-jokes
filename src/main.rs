use windows::UI::Notifications::{ToastNotification, ToastNotifier};
use windows::Data::Xml::Dom::XmlDocument;

use windows::core::Result;
use windows::core::HSTRING;
use windows::UI::Notifications::ToastNotificationManager;

fn main() -> Result<()> {
    let message = "Hallo!";
    let xml = format!(
        r#"
        <toast>
            <visual>
                <binding template="ToastText01">
                    <text id="1">Dies ist eine Toast-Benachrichtigung</text>
                </binding>
            </visual>
        </toast>

    "#
    );

    // Parsen der XML-Daten
    let xml_document = XmlDocument::new()?;
    xml_document.LoadXml(&HSTRING::from(&xml))?;

    // Erstellen der Benachrichtigung
    let toast_notification = ToastNotification::CreateToastNotification(&xml_document)?;

    let notifier = ToastNotificationManager::CreateToastNotifier()?;
    notifier.Show(&toast_notification)?;
    Ok(())
}
