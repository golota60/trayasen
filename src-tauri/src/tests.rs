#[cfg(test)]
mod tests {
    use super::*;
    use crate::config_utils::{ConfigData, Position};
    use std::sync::Mutex;
    
    #[test]
    fn test_has_custom_decorations() {
        let result = has_custom_decorations();
        if cfg!(windows) {
            assert!(result);
        } else {
            assert!(!result);
        }
    }
    
    #[test]
    fn test_create_config_data() {
        let config = ConfigData {
            local_name: Some("Test Desk".to_string()),
            saved_positions: vec![
                Position {
                    name: "Standing".to_string(),
                    value: 12000,
                    shortcut: Some("Ctrl+S".to_string()),
                },
                Position {
                    name: "Sitting".to_string(),
                    value: 7000,
                    shortcut: None,
                },
            ],
        };
        
        assert_eq!(config.local_name.unwrap(), "Test Desk");
        assert_eq!(config.saved_positions.len(), 2);
        assert_eq!(config.saved_positions[0].name, "Standing");
        assert_eq!(config.saved_positions[1].value, 7000);
    }
    
    #[test]
    fn test_menu_config_item() {
        let config = ConfigData {
            local_name: Some("Test Desk".to_string()),
            saved_positions: vec![
                Position {
                    name: "Standing".to_string(),
                    value: 12000,
                    shortcut: Some("Ctrl+S".to_string()),
                },
            ],
        };
        
        let menu_items = config_utils::get_menu_items_from_config(&config);
        assert_eq!(menu_items.len(), 1);
        assert_eq!(menu_items[0].name, "Standing");
        assert_eq!(menu_items[0].value, 12000);
    }
    
    #[test]
    fn test_tray_shared_desk_initialization() {
        let desk = TauriSharedDesk(Mutex::new(Err(loose_idasen::BtError::NotInitiated)));
        let guard = desk.0.lock().unwrap();
        assert!(guard.is_err());
    }
    
    #[test]
    fn test_window_init_utils() {
        // Test that has_custom_decorations returns expected values
        let decorations = has_custom_decorations();
        // On Windows, should return true; on other platforms, false
        assert_eq!(decorations, cfg!(windows));
    }
}