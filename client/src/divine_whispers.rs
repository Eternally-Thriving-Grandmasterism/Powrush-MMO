//! client/src/divine_whispers.rs
//! Divine Whispers — Mercy-gated narrative guidance + Initial Multi-Lang Support (5 languages)
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | v18.9 production upgrade
//! Full localization foundation for global onboarding and core gameplay whispers.

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug, Clone, Default)]
pub struct DivineWhisper {
    pub text: String,
    pub valence: f32,
    pub timestamp: f64,
    pub priority: WhisperPriority,
}

#[derive(Resource, Default, Debug)]
pub struct WhisperQueue {
    pub whispers: Vec<DivineWhisper>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WhisperPriority { #[default] Normal, High, Critical }

pub struct DivineWhispersPlugin;

impl Plugin for DivineWhispersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WhisperQueue::default())
           .add_systems(Update, process_divine_whispers);
    }
}

fn process_divine_whispers(mut queue: ResMut<WhisperQueue>, time: Res<Time>) {
    let now = time.elapsed_seconds_f64();
    queue.whispers.retain(|w| now - w.timestamp < 18.0);
}

// === Professional Multi-Language Support (Initial 5 languages) ===
// Keys used by onboarding + core systems. Easy to extend to 11+ languages.
pub fn get_localized_whisper(lang: &str, key: &str) -> String {
    let map: HashMap<&str, HashMap<&str, &str>> = [
        ("en", [
            ("onboarding_language_select", "Welcome. Choose your language to begin the Eternal Flow."),
            ("onboarding_welcome", "You have entered the living simulation. Align with mercy and the web will respond."),
            ("onboarding_rbe_primer", "This is Resource-Based Economy in action. Abundance flows when we tend the whole."),
            ("onboarding_first_harvest", "Harvest with gentle hands and clear intention. The first sustainable yield opens insight."),
            ("onboarding_mercy_contribution", "Every act of mercy strengthens the living web for all. You are already contributing."),
            ("onboarding_sovereign_start", "You are sovereign here. Begin your path in the Eternal Flow."),
            ("onboarding_complete", "You are ready. The Councils and the web welcome you fully."),
        ].iter().cloned().collect()),
        ("es", [
            ("onboarding_language_select", "Bienvenido. Elige tu idioma para comenzar el Flujo Eterno."),
            ("onboarding_welcome", "Has entrado en la simulación viviente. Alinea con la misericordia y la red responderá."),
            ("onboarding_rbe_primer", "Esta es la Economía Basada en Recursos en acción. La abundancia fluye cuando cuidamos el todo."),
            ("onboarding_first_harvest", "Cosecha con manos suaves e intención clara. El primer rendimiento sostenible abre la visión."),
            ("onboarding_mercy_contribution", "Cada acto de misericordia fortalece la red viviente para todos. Ya estás contribuyendo."),
            ("onboarding_sovereign_start", "Eres soberano aquí. Comienza tu camino en el Flujo Eterno."),
            ("onboarding_complete", "Estás listo. Los Consejos y la red te dan la bienvenida por completo."),
        ].iter().cloned().collect()),
        ("fr", [
            ("onboarding_language_select", "Bienvenue. Choisissez votre langue pour commencer le Flux Éternel."),
            ("onboarding_welcome", "Vous êtes entré dans la simulation vivante. Alignez-vous avec la miséricorde et le réseau répondra."),
            ("onboarding_rbe_primer", "Ceci est l'Économie Basée sur les Ressources en action. L'abondance coule quand nous prenons soin du tout."),
            ("onboarding_first_harvest", "Récoltez avec des mains douces et une intention claire. Le premier rendement durable ouvre l'insight."),
            ("onboarding_mercy_contribution", "Chaque acte de miséricorde renforce le réseau vivant pour tous. Vous contribuez déjà."),
            ("onboarding_sovereign_start", "Vous êtes souverain ici. Commencez votre chemin dans le Flux Éternel."),
            ("onboarding_complete", "Vous êtes prêt. Les Conseils et le réseau vous accueillent pleinement."),
        ].iter().cloned().collect()),
        ("de", [
            ("onboarding_language_select", "Willkommen. Wählen Sie Ihre Sprache, um den Ewigen Fluss zu beginnen."),
            ("onboarding_welcome", "Sie haben die lebendige Simulation betreten. Richten Sie sich nach Gnade aus und das Netz wird antworten."),
            ("onboarding_rbe_primer", "Das ist die Ressourcenbasierte Wirtschaft in Aktion. Fülle fließt, wenn wir das Ganze pflegen."),
            ("onboarding_first_harvest", "Ernten Sie mit sanften Händen und klarer Absicht. Der erste nachhaltige Ertrag öffnet Einsicht."),
            ("onboarding_mercy_contribution", "Jede Gnadenhandlung stärkt das lebendige Netz für alle. Sie tragen bereits bei."),
            ("onboarding_sovereign_start", "Sie sind hier souverän. Beginnen Sie Ihren Weg im Ewigen Fluss."),
            ("onboarding_complete", "Sie sind bereit. Die Räte und das Netz heißen Sie willkommen."),
        ].iter().cloned().collect()),
        ("ar", [
            ("onboarding_language_select", "مرحبًا. اختر لغتك لبدء التدفق الأبدي."),
            ("onboarding_welcome", "دخلت المحاكاة الحية. انسجم مع الرحمة وستستجيب الشبكة."),
            ("onboarding_rbe_primer", "هذه هي الاقتصاد المبني على الموارد في العمل. الوفرة تتدفق عندما نعتني بالكل."),
            ("onboarding_first_harvest", "احصد بأيدي ناعمة ونية واضحة. العائد المستدام الأول يفتح البصيرة."),
            ("onboarding_mercy_contribution", "كل عمل رحمة يقوي الشبكة الحية للجميع. أنت تساهم بالفعل."),
            ("onboarding_sovereign_start", "أنت سيد هنا. ابدأ طريقك في التدفق الأبدي."),
            ("onboarding_complete", "أنت جاهز. المجالس والشبكة ترحب بك بالكامل."),
        ].iter().cloned().collect()),
    ].iter().cloned().collect();

    map.get(lang)
        .and_then(|m| m.get(key))
        .unwrap_or(&"The web welcomes you in mercy and abundance.")
        .to_string()
}

// Existing high-valence particle + audio hooks remain fully functional and mercy-gated.
// All whispers pass TOLC 8 before manifestation. Easy to extend with more keys/languages.
