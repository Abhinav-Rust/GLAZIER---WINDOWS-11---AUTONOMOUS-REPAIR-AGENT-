use crate::wrl::ast::{VirodhaNode, Action, UpayaType};

pub struct UpayaResolver;

impl UpayaResolver {
    pub fn resolve(virodha: &VirodhaNode) -> Vec<Action> {
        let mut plan = Vec::new();
        // Order by severity: Sama (Conciliation), Dana (Concession), Bheda (Division), Danda (Force)
        for expected in [UpayaType::Sama, UpayaType::Dana, UpayaType::Bheda, UpayaType::Danda] {
            if let Some(item) = virodha.upaya.iter().find(|i| i.upaya_type == expected) {
                plan.push(item.action.clone());
            }
        }
        plan
    }
}
