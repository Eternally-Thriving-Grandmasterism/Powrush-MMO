use std::collections::HashMap;

/// Education Quest System
/// Real-world skill learning → grace points + income potential
/// Mercy: knowledge shared freely, growth celebrated

#[derive(Clone)]
pub struct EducationQuest {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub category: QuestCategory,
    pub difficulty: u8,              // 1–5
    pub grace_reward: u64,
    pub completed_by: Vec<String>,   // player_ids
}

#[derive(Clone)]
pub enum QuestCategory {
    Science,
    Sustainability,
    Empathy,
    CriticalThinking,
    ArtCreativity,
    HistoryWisdom,
}

pub struct QuestSystem {
    quests: HashMap<u64, EducationQuest>,
    player_progress: HashMap<String, Vec<u64>>, // player_id -> completed quest ids
    next_id: u64,
}

impl QuestSystem {
    pub fn new() -> Self {
        let mut system = Self {
            quests: HashMap::new(),
            player_progress: HashMap::new(),
            next_id: 1,
        };
        system.initialize_quests();
        system
    }

    fn initialize_quests(&mut self) {
        self.add_quest(
            "The Breath of Life",
            "Learn diaphragmatic breathing and its effects on nervous system regulation. Practice 10 mindful breaths.",
            QuestCategory::Empathy,
            1,
            100,
        );
        self.add_quest(
            "Water is Life",
            "Study local water cycles and design a simple rainwater collection system. Share your blueprint.",
            QuestCategory::Sustainability,
            2,
            300,
        );
        self.add_quest(
            "Stellar Navigation",
            "Identify 5 constellations and explain how ancient cultures used them for timekeeping.",
            QuestCategory::Science,
            2,
            250,
        );
        self.add_quest(
            "The Art of Listening",
            "Practice active listening in 3 conversations. Reflect on what you learned about the speaker.",
            QuestCategory::Empathy,
            3,
            500,
        );
        self.add_quest(
            "Zero Waste Challenge",
            "Track your waste for 7 days and propose 3 practical reductions for your community.",
            QuestCategory::Sustainability,
            3,
            600,
        );
    }

    fn add_quest(&mut self, title: &str, desc: &str, cat: QuestCategory, diff: u8, reward: u64) {
        let quest = EducationQuest {
            id: self.next_id,
            title: title.to_string(),
            description: desc.to_string(),
            category: cat,
            difficulty: diff,
            grace_reward: reward,
            completed_by: Vec::new(),
        };
        self.quests.insert(self.next_id, quest);
        self.next_id += 1;
    }

    pub fn list_available(&self) -> Vec<&EducationQuest> {
        self.quests.values().collect()
    }

    pub fn complete_quest(&mut self, player_id: &str, quest_id: u64) -> Result<u64, String> {
        if let Some(quest) = self.quests.get(&quest_id) {
            let player_key = player_id.to_string();
            let progress = self.player_progress.entry(player_key.clone()).or_insert_with(Vec::new);
            
            if progress.contains(&quest_id) {
                return Err("Quest already completed".to_string());
            }
            
            progress.push(quest_id);
            quest.completed_by.push(player_key);
            
            println!("{} completed: {} — +{} grace points!", player_id, quest.title, quest.grace_reward);
            Ok(quest.grace_reward)
        } else {
            Err("Quest not found".to_string())
        }
    }

    pub fn player_grace_from_quests(&self, player_id: &str) -> u64 {
        let progress = self.player_progress.get(player_id).unwrap_or(&Vec::new());
        progress.iter()
            .filter_map(|&qid| self.quests.get(&qid))
            .map(|q| q.grace_reward)
            .sum()
    }
}

// Example usage
pub fn example_education() {
    let mut quests = QuestSystem::new();
    
    println!("Available Education Quests:");
    for q in quests.list_available() {
        println!("{} [{}]: {} (Reward: {} grace)", q.id, format!("{:?}", q.category), q.title, q.grace_reward);
    }
    
    // Player completes quest
    quests.complete_quest("player_123", 1);
    quests.complete_quest("player_123", 2);
    
    let grace = quests.player_grace_from_quests("player_123");
    println!("Player_123 total education grace: {}", grace);
}
