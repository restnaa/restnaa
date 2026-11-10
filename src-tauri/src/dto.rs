use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum StorageStatus {
    Ready {
        #[serde(rename = "schemaVersion")]
        schema_version: u32,
    },
    Unavailable {
        message: String,
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub storage: StorageStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_matches_frontend_wire_contract() {
        let info = AppInfo {
            version: "0.1.0".into(),
            storage: StorageStatus::Ready { schema_version: 1 },
        };
        assert_eq!(
            serde_json::to_value(info).unwrap(),
            serde_json::json!({
                "version": "0.1.0", "storage": { "status": "ready", "schemaVersion": 1 }
            })
        );
        assert_eq!(
            serde_json::to_value(StorageStatus::Unavailable {
                message: "Storage unavailable".into()
            })
            .unwrap(),
            serde_json::json!({
                "status": "unavailable", "message": "Storage unavailable"
            })
        );
    }

    #[test]
    fn scheduler_inputs_reject_unknown_fields_and_wire_names_are_camel_case() {
        use crate::{
            scheduler::{model::PauseRequest, service::StateEvent},
            settings::Settings,
        };
        assert!(
            serde_json::from_value::<Settings>(serde_json::json!({"eyeEnabled":true})).is_err()
        );
        let mut value = serde_json::to_value(Settings::default()).unwrap();
        assert_eq!(value["eyeIntervalMinutes"], 20);
        assert_eq!(value["workStartLocal"], "09:00:00");
        value["unrecognizedPreference"] = serde_json::json!(true);
        assert!(serde_json::from_value::<Settings>(value).is_err());
        assert!(serde_json::from_value::<PauseRequest>(
            serde_json::json!({"kind":"manual","extra":true})
        )
        .is_err());
        assert!(matches!(
            serde_json::from_value::<PauseRequest>(serde_json::json!({"kind":"untilTomorrow"}))
                .unwrap(),
            PauseRequest::UntilTomorrow {}
        ));
        assert_eq!(
            serde_json::to_value(StateEvent::Error {
                message: "Storage unavailable".into()
            })
            .unwrap(),
            serde_json::json!({"kind":"error","message":"Storage unavailable"})
        );
    }
}
