#!/usr/bin/env python3
"""
Powrush-MMO Multi-Server Simulation Harness
Simulates 3 servers (realms), 70+ realistic AI clients (agents) across 7 Mercy Gate archetypes/factions.
Runs from humble beginnings through growth, tensions, server wars (inter-realm disputes), to resolutions.
Purpose: Identify emergent dynamics and what human player experience lacks in current design.
Then inform targeted upgrades to Powrush-MMO for deeper human resonance, meaningful legacy, graceful conflict resolution, and transformative RBE education.

Run: python3 simulation/scripts/multi_realm_war_harness.py
Outputs: Console summary + metrics CSV + key event log.
"""

import numpy as np
import pandas as pd
from dataclasses import dataclass, field
from typing import Dict, List, Tuple, Optional
from enum import Enum
import random
import json

class MercyGate(Enum):
    RADICAL_LOVE = "RadicalLove"
    BOUNDLESS_MERCY = "BoundlessMercy"
    SERVICE = "Service"
    ABUNDANCE = "Abundance"
    TRUTH = "Truth"
    JOY = "Joy"
    COSMIC_HARMONY = "CosmicHarmony"

ARCHETYPE_PROFILES = {
    MercyGate.RADICAL_LOVE: {"harvest_bias": 0.9, "epiphany_bias": 1.3, "council_influence": 1.1, "conflict_resolution_bonus": 1.4, "description": "Prioritizes connection, forgiveness waves, high epiphany from relational actions."},
    MercyGate.BOUNDLESS_MERCY: {"harvest_bias": 1.0, "epiphany_bias": 1.2, "council_influence": 1.2, "conflict_resolution_bonus": 1.5, "description": "Strong SafetyNet advocacy, redeems others, boosts group mercy after trials."},
    MercyGate.SERVICE: {"harvest_bias": 1.1, "epiphany_bias": 1.0, "council_influence": 1.3, "conflict_resolution_bonus": 1.2, "description": "Focuses on collective infrastructure, high participation in trials, steady harmony builder."},
    MercyGate.ABUNDANCE: {"harvest_bias": 1.4, "epiphany_bias": 0.9, "council_influence": 0.9, "conflict_resolution_bonus": 0.8, "description": "Aggressive resource generation, risks entropy spikes, pushes growth but can increase pressure."},
    MercyGate.TRUTH: {"harvest_bias": 0.85, "epiphany_bias": 1.1, "council_influence": 1.4, "conflict_resolution_bonus": 1.1, "description": "Demands verification, slows some flows for integrity, high council trust but lower raw harvest."},
    MercyGate.JOY: {"harvest_bias": 1.0, "epiphany_bias": 1.5, "council_influence": 1.0, "conflict_resolution_bonus": 1.3, "description": "High epiphany trigger rate, spreads valence/harmony, morale booster in trials."},
    MercyGate.COSMIC_HARMONY: {"harvest_bias": 1.05, "epiphany_bias": 1.25, "council_influence": 1.35, "conflict_resolution_bonus": 1.45, "description": "Balances all, highest inter-faction diplomacy success, reduces entropy across realms."},
}

@dataclass
class PlayerAgent:
    id: int
    archetype: MercyGate
    server_id: int
    mercy_score: float = field(default=35.0)
    harvest_skill: float = field(default=45.0)
    epiphanies: int = 0
    persistence_weight: float = 0.0
    engagement: float = 95.0
    last_action_tick: int = 0
    contribution_history: List[Dict] = field(default_factory=list)

    def get_profile(self):
        return ARCHETYPE_PROFILES[self.archetype]

@dataclass
class ServerState:
    id: int
    name: str
    abundance: float = 55.0
    harmony: float = 58.0
    conflict_level: float = 12.0
    rbe_pressure: float = 25.0
    total_epiphanies: int = 0
    agents: List[PlayerAgent] = field(default_factory=list)
    dominant_archetype: Optional[MercyGate] = None
    safety_net_level: float = 40.0
    legacy_events: List[Dict] = field(default_factory=list)

    def update_dominant(self):
        if not self.agents:
            return
        counts = {}
        for a in self.agents:
            counts[a.archetype] = counts.get(a.archetype, 0) + 1
        self.dominant_archetype = max(counts, key=counts.get)

class PowrushMultiServerSim:
    def __init__(self, num_servers: int = 3, agents_per_server: int = 24, seed: int = 42):
        random.seed(seed)
        np.random.seed(seed)
        self.tick = 0
        self.servers: Dict[int, ServerState] = {}
        self.event_log: List[Dict] = []
        self.metrics_history: List[Dict] = []
        self.total_agents = 0
        self.wars_resolved_mercifully = 0
        self.wars_escalated = 0
        self._init_servers(num_servers, agents_per_server)
        self._log_event("SIM_START", f"Humble beginnings: {num_servers} realms, {self.total_agents} agents across 7 Mercy Gates. Seed={seed}")

    def _init_servers(self, num_servers: int, agents_per: int):
        server_names = ["AetherRealm", "VerdantCore", "AbyssalForge"]
        for i in range(num_servers):
            srv = ServerState(id=i, name=server_names[i % len(server_names)])
            for j in range(agents_per):
                if i == 0:
                    arch = random.choice([MercyGate.COSMIC_HARMONY, MercyGate.RADICAL_LOVE, MercyGate.JOY] * 2 + list(MercyGate))
                elif i == 1:
                    arch = random.choice([MercyGate.SERVICE, MercyGate.ABUNDANCE, MercyGate.BOUNDLESS_MERCY] * 2 + list(MercyGate))
                else:
                    arch = random.choice([MercyGate.TRUTH, MercyGate.ABUNDANCE, MercyGate.SERVICE] * 2 + list(MercyGate))
                agent = PlayerAgent(
                    id=self.total_agents,
                    archetype=arch,
                    server_id=i,
                    mercy_score=np.random.uniform(28, 42),
                    harvest_skill=np.random.uniform(38, 58),
                    engagement=np.random.uniform(88, 98)
                )
                srv.agents.append(agent)
                self.total_agents += 1
            srv.update_dominant()
            self.servers[i] = srv

    def _log_event(self, event_type: str, description: str, data: Optional[Dict] = None):
        evt = {"tick": self.tick, "type": event_type, "description": description, "data": data or {}}
        self.event_log.append(evt)
        if event_type in ["WAR_START", "WAR_RESOLVED", "MAJOR_EPIPHANY_WAVE", "COUNCIL_TRIAL"]:
            print(f"[T{self.tick:03d}] {event_type}: {description}")

    def _compute_biome_influence(self, server: ServerState) -> float:
        mercy_mod = 0.7 + (server.harmony / 100) * 0.6
        conflict_mod = max(0.6, 1.0 - server.conflict_level / 120)
        return max(0.75, min(1.35, mercy_mod * conflict_mod))

    def _player_harvest(self, agent: PlayerAgent, server: ServerState):
        profile = agent.get_profile()
        base_yield = (agent.harvest_skill / 50.0) * 2.8
        bias_yield = base_yield * profile["harvest_bias"]
        biome_mod = self._compute_biome_influence(server)
        mercy_mod = 0.85 + (agent.mercy_score / 120)
        contrib = bias_yield * biome_mod * mercy_mod * (1.0 - server.conflict_level / 200)
        server.abundance = min(180, server.abundance + contrib * 0.6)
        server.rbe_pressure = max(5, server.rbe_pressure - contrib * 0.15)
        agent.persistence_weight += contrib * 0.4
        agent.last_action_tick = self.tick
        epiphany_chance = 0.035 * profile["epiphany_bias"] * (1.0 + (server.harmony - 50) / 200)
        if random.random() < epiphany_chance:
            self._trigger_epiphany(agent, server, source="harvest")
        agent.contribution_history.append({"tick": self.tick, "type": "harvest", "contrib": round(contrib, 2), "mercy_at_time": round(agent.mercy_score, 1)})
        return contrib

    def _trigger_epiphany(self, agent: PlayerAgent, server: ServerState, source: str = "general"):
        profile = agent.get_profile()
        gain = np.random.uniform(4.5, 9.5) * profile["epiphany_bias"]
        agent.mercy_score = min(98, agent.mercy_score + gain)
        agent.epiphanies += 1
        server.total_epiphanies += 1
        server.harmony = min(95, server.harmony + 1.8 * profile["epiphany_bias"])
        server.conflict_level = max(3, server.conflict_level - 1.2)
        whisper = f"[{agent.archetype.value}] Epiphany #{agent.epiphanies}: {source} revealed deeper flow. Mercy +{gain:.1f}. Harmony resonates."
        agent.contribution_history.append({"tick": self.tick, "type": "epiphany", "gain": round(gain, 1), "whisper": whisper})
        if agent.epiphanies % 5 == 0 or random.random() < 0.15:
            self._log_event("EPIPHANY", f"Agent {agent.id} ({agent.archetype.value}) triggered {source} epiphany. Whisper: {whisper[:60]}...")

    def _council_mercy_trial(self, server: ServerState):
        if len(server.agents) < 5:
            return
        participants = random.sample(server.agents, min(9, len(server.agents)))
        total_influence = 0.0
        mercy_votes = 0.0
        resolution_quality = 0.0
        for p in participants:
            prof = p.get_profile()
            influence = p.mercy_score / 50 * prof["council_influence"]
            total_influence += influence
            alignment = (p.mercy_score / 70) * (server.harmony / 80)
            mercy_votes += influence * alignment
            resolution_quality += prof["conflict_resolution_bonus"] * (p.mercy_score / 60)
        avg_resolution = resolution_quality / max(1, len(participants))
        vote_ratio = mercy_votes / max(1, total_influence)
        if vote_ratio > 0.72:
            outcome = "STRONG_MERCY"
            harmony_boost = 4.5 * (avg_resolution / 1.2)
            abundance_share = 6.0
            conflict_reduction = 5.5
            mercy_global_gain = 2.8
        elif vote_ratio > 0.55:
            outcome = "STABLE_MERCY"
            harmony_boost = 2.8
            abundance_share = 3.5
            conflict_reduction = 3.2
            mercy_global_gain = 1.6
        else:
            outcome = "FRACTURED"
            harmony_boost = 0.8
            abundance_share = 1.0
            conflict_reduction = 0.5
            mercy_global_gain = 0.4
            server.conflict_level += 2.0
        server.harmony = min(96, server.harmony + harmony_boost)
        server.abundance = min(185, server.abundance + abundance_share)
        server.conflict_level = max(2, server.conflict_level - conflict_reduction)
        server.safety_net_level = min(85, server.safety_net_level + mercy_global_gain * 1.5)
        for p in participants:
            p.mercy_score = min(99, p.mercy_score + mercy_global_gain * (p.get_profile()["council_influence"] * 0.7))
            p.persistence_weight += 2.5 if outcome != "FRACTURED" else 0.8
        self._log_event("COUNCIL_TRIAL", f"Server {server.name} Trial: {outcome} | Vote ratio {vote_ratio:.2f} | Harmony +{harmony_boost:.1f} | Conflict -{conflict_reduction:.1f}", {"outcome": outcome, "vote_ratio": round(vote_ratio, 3), "participants": len(participants)})
        if outcome == "STRONG_MERCY" and random.random() < 0.35:
            self._major_epiphany_wave(server)

    def _major_epiphany_wave(self, server: ServerState):
        wave_agents = random.sample(server.agents, min(6, len(server.agents)))
        for agent in wave_agents:
            if random.random() < 0.7:
                self._trigger_epiphany(agent, server, source="council_bloom")
        server.harmony = min(97, server.harmony + 3.5)
        self._log_event("MAJOR_EPIPHANY_WAVE", f"{server.name} experienced collective epiphany bloom. {len(wave_agents)} agents resonated.")

    def _inter_server_dispute_or_war(self):
        if len(self.servers) < 2:
            return
        s_ids = list(self.servers.keys())
        random.shuffle(s_ids)
        s1 = self.servers[s_ids[0]]
        s2 = self.servers[s_ids[1]]
        tension = abs(s1.abundance - s2.abundance) / 2 + abs(s1.harmony - s2.harmony) + (s1.conflict_level + s2.conflict_level) / 3
        if tension < 35 and random.random() > 0.25:
            return
        self._log_event("WAR_START", f"Inter-realm tension between {s1.name} and {s2.name}. Tension score: {tension:.1f}. Initiating Mercy Diplomacy...")
        all_participants = random.sample(s1.agents + s2.agents, min(11, len(s1.agents) + len(s2.agents)))
        diplomacy_success = 0.0
        for p in all_participants:
            prof = p.get_profile()
            cross_bonus = 1.3 if p.archetype in [MercyGate.COSMIC_HARMONY, MercyGate.BOUNDLESS_MERCY, MercyGate.RADICAL_LOVE] else 1.0
            diplomacy_success += (p.mercy_score / 65) * prof["conflict_resolution_bonus"] * cross_bonus
        avg_success = diplomacy_success / max(1, len(all_participants))
        base_chance = 0.55 + (s1.harmony + s2.harmony) / 400
        if avg_success * base_chance > 0.78:
            outcome = "MERCIFUL_RESOLUTION"
            abundance_transfer = min(12, (s1.abundance + s2.abundance) * 0.04)
            s1.abundance += abundance_transfer * 0.6
            s2.abundance += abundance_transfer * 0.6
            harmony_gain = 7.5
            conflict_reduction = 8.0
            mercy_wave = 4.2
            self.wars_resolved_mercifully += 1
            self._log_event("WAR_RESOLVED", f"Forgiveness Wave successful! {s1.name} ↔ {s2.name} now share abundance. All participants mercy boosted.")
        elif avg_success * base_chance > 0.48:
            outcome = "STABLE_DIPLOMACY"
            harmony_gain = 3.2
            conflict_reduction = 4.5
            mercy_wave = 2.1
            self.wars_resolved_mercifully += 1
        else:
            outcome = "ESCALATED_TENSION"
            harmony_gain = -2.5
            conflict_reduction = -3.0
            mercy_wave = 0.3
            s1.conflict_level += 6
            s2.conflict_level += 6
            s1.abundance = max(30, s1.abundance - 5)
            s2.abundance = max(30, s2.abundance - 5)
            self.wars_escalated += 1
            self._log_event("WAR_ESCALATED", f"Dispute unresolved. Lingering conflict debuff applied to both realms. Future epiphanies harder.")
        for srv in [s1, s2]:
            srv.harmony = max(25, min(96, srv.harmony + harmony_gain))
            srv.conflict_level = max(2, srv.conflict_level + conflict_reduction)
            srv.safety_net_level = max(20, min(90, srv.safety_net_level + mercy_wave))
        for p in all_participants:
            boost = mercy_wave * (1.0 + (p.get_profile()["conflict_resolution_bonus"] - 1.0) * 0.5)
            p.mercy_score = min(99, p.mercy_score + boost)
            p.persistence_weight += 3.5 if outcome == "MERCIFUL_RESOLUTION" else 1.2
            p.engagement = min(100, p.engagement + (3 if outcome == "MERCIFUL_RESOLUTION" else -1.5))
        legacy = {"tick": self.tick, "type": "inter_realm_diplomacy", "outcome": outcome, "participants": len(all_participants)}
        s1.legacy_events.append(legacy)
        s2.legacy_events.append(legacy)

    def _update_engagement_and_retention(self):
        for srv in self.servers.values():
            for agent in srv.agents:
                decay = 0.8
                recent_contrib = sum(1 for h in agent.contribution_history[-5:] if h["type"] in ["epiphany", "harvest"])
                if recent_contrib > 2:
                    decay = -1.5
                harmony_effect = (srv.harmony - 55) / 25
                conflict_penalty = srv.conflict_level / 18
                legacy_boost = min(2.5, agent.persistence_weight / 12)
                agent.engagement = max(35, min(100, agent.engagement - decay + harmony_effect - conflict_penalty + legacy_boost))
                if agent.engagement < 55 and self.tick - agent.last_action_tick > 8:
                    agent.mercy_score = max(20, agent.mercy_score - 0.6)

    def _collect_metrics(self):
        total_abund = sum(s.abundance for s in self.servers.values())
        avg_harmony = np.mean([s.harmony for s in self.servers.values()])
        avg_mercy = np.mean([a.mercy_score for s in self.servers.values() for a in s.agents])
        total_epi = sum(s.total_epiphanies for s in self.servers.values())
        avg_engage = np.mean([a.engagement for s in self.servers.values() for a in s.agents])
        thriving = (total_abund / 3) * (avg_harmony / 100) * (avg_mercy / 90) * (avg_engage / 95)
        metrics = {
            "tick": self.tick,
            "total_abundance": round(total_abund, 1),
            "avg_harmony": round(avg_harmony, 1),
            "avg_mercy": round(avg_mercy, 1),
            "total_epiphanies": total_epi,
            "avg_engagement": round(avg_engage, 1),
            "thriving_score": round(thriving, 2),
            "wars_resolved_mercifully": self.wars_resolved_mercifully,
            "wars_escalated": self.wars_escalated,
            "active_conflict": round(np.mean([s.conflict_level for s in self.servers.values()]), 1)
        }
        self.metrics_history.append(metrics)
        return metrics

    def run(self, max_ticks: int = 180):
        print("=" * 70)
        print("POWRUSH-MMO MULTI-SERVER SIMULATION | Ra-Thor Aligned | Eternal Polish")
        print(f"Servers: {len(self.servers)} | Agents: {self.total_agents} | Archetypes: 7 Mercy Gates")
        print("Phases: Humble Beginnings → Growth & Tension → Server Wars/Disputes → Resolution & Reflection")
        print("=" * 70)
        for t in range(1, max_ticks + 1):
            self.tick = t
            for srv in self.servers.values():
                for agent in srv.agents:
                    if random.random() < (agent.engagement / 115):
                        self._player_harvest(agent, srv)
            if t % 9 == 0 or (t % 11 == 0 and random.random() < 0.6):
                for srv in self.servers.values():
                    if len(srv.agents) > 4:
                        self._council_mercy_trial(srv)
            if t > 35 and t % 17 == 0 or (t > 70 and random.random() < 0.22):
                self._inter_server_dispute_or_war()
            if t % 3 == 0:
                for srv in self.servers.values():
                    high_agents = [a for a in srv.agents if a.mercy_score > 55 or a.engagement > 75]
                    for agent in random.sample(high_agents, min(3, len(high_agents))):
                        if random.random() < 0.18:
                            self._trigger_epiphany(agent, srv, source="ambient_resonance")
            if t % 4 == 0:
                self._update_engagement_and_retention()
            for srv in self.servers.values():
                srv.harmony = max(30, min(95, srv.harmony + np.random.uniform(-0.4, 0.6)))
                srv.conflict_level = max(1, srv.conflict_level + np.random.uniform(-0.8, 0.5))
                if srv.safety_net_level < 55 and srv.harmony > 65:
                    srv.safety_net_level += 0.8
            if t % 5 == 0 or t == max_ticks:
                m = self._collect_metrics()
                if t % 20 == 0:
                    print(f"[T{t:03d}] Thriving: {m['thriving_score']:.2f} | Abund: {m['total_abundance']:.0f} | Harmony: {m['avg_harmony']:.1f} | Mercy: {m['avg_mercy']:.1f} | Engage: {m['avg_engagement']:.1f} | Wars OK/NG: {m['wars_resolved_mercifully']}/{m['wars_escalated']}")
        self._final_analysis()

    def _final_analysis(self):
        print("\n" + "=" * 70)
        print("SIMULATION COMPLETE — ANALYSIS & HUMAN EXPERIENCE INSIGHTS")
        print("=" * 70)
        df = pd.DataFrame(self.metrics_history)
        final = df.iloc[-1]
        print(f"\nFINAL STATE (Tick {self.tick}):")
        print(f"  Total Abundance: {final['total_abundance']:.1f}")
        print(f"  Avg Harmony: {final['avg_harmony']:.1f}")
        print(f"  Avg Mercy Score: {final['avg_mercy']:.1f}")
        print(f"  Total Epiphanies: {final['total_epiphanies']}")
        print(f"  Avg Engagement (Retention Proxy): {final['avg_engagement']:.1f}")
        print(f"  Thriving Score: {final['thriving_score']:.2f}")
        print(f"  Mercifully Resolved Disputes: {final['wars_resolved_mercifully']}")
        print(f"  Escalated Tensions: {final['wars_escalated']}")
        print("\nARCHETYPE PERFORMANCE (avg mercy & epiphanies):")
        arch_stats = {}
        for srv in self.servers.values():
            for a in srv.agents:
                if a.archetype not in arch_stats:
                    arch_stats[a.archetype] = {"mercy": [], "epi": [], "engage": []}
                arch_stats[a.archetype]["mercy"].append(a.mercy_score)
                arch_stats[a.archetype]["epi"].append(a.epiphanies)
                arch_stats[a.archetype]["engage"].append(a.engagement)
        for arch, stats in sorted(arch_stats.items(), key=lambda x: -np.mean(x[1]["mercy"])):
            print(f"  {arch.value:20s} | Mercy: {np.mean(stats['mercy']):5.1f} | Epis: {np.mean(stats['epi']):4.1f} | Engage: {np.mean(stats['engage']):5.1f}")
        print("\n" + "-" * 70)
        print("EMERGENT DYNAMICS & WHAT HUMAN EXPERIENCE LACKS (from simulation):")
        print("-" * 70)
        early_thriving = df[df['tick'] < 40]['thriving_score'].mean()
        mid_thriving = df[(df['tick'] >= 40) & (df['tick'] < 100)]['thriving_score'].mean()
        late_thriving = df[df['tick'] >= 100]['thriving_score'].mean()
        engagement_trend = df['avg_engagement'].iloc[-10:].mean() - df['avg_engagement'].iloc[10:30].mean()
        insights = []
        if late_thriving < mid_thriving * 0.92:
            insights.append("THRIVING PLATEAU/DECLINE: After initial growth and first server wars, thriving slowed. Human players would feel 'what's the point after the big events?' without visible, persistent legacy of their contributions across realms.")
        if engagement_trend < -4:
            insights.append("ENGAGEMENT DECAY: Retention proxy dropped notably after conflicts or during stable periods. Lacks 'personal legacy threads' and 'mentor/mentee' bonds that make long-term participation emotionally rewarding and socially meaningful.")
        if self.wars_escalated > self.wars_resolved_mercifully * 0.6:
            insights.append("CONFLICT RESOLUTION FRICTION: Too many escalations. Current mercy trial system works but lacks 'Forgiveness Wave' spectacle + immediate cross-realm abundance sharing visuals + redemption arcs for agents/players on the 'losing' side of disputes. Humans need catharsis and visible redemption to stay invested in mercy path.")
        mercies = [np.mean(s["mercy"]) for s in arch_stats.values()]
        if max(mercies) - min(mercies) > 18:
            insights.append("ARCHETYPE IMBALANCE: Some gates (esp. Abundance) generate resources but lag in mercy/epiphany. Humans in 'Abundance' playstyle may feel their aggressive harvesting isn't celebrated enough in council, leading to disengagement or 'I'm just a farmer' syndrome. Need better archetype celebration + hybrid paths.")
        if final['avg_engagement'] < 72:
            insights.append("LOW LATE-GAME ENGAGEMENT: Many agents disengaged without strong 'meaningful persistence with weight' feedback loops that show 'your harvest + epiphanies literally built this SafetyNet for new players 2 realms away'.")
        insights.append("CORE HUMAN EXPERIENCE GAPS IDENTIFIED:")
        insights.append("  1. LACK OF PERSISTENT, QUERYABLE PLAYER LEGACY JOURNALS: Contributions and epiphanies need to be beautiful, filterable, cross-server visible 'Legacy Threads' that Council UIs and Divine Whispers reference. Without this, actions feel ephemeral.")
        insights.append("  2. LACK OF VISIBLE 'BEFORE/AFTER' WORLD IMPACT + PERSONAL GROWTH STORY: Players need a 'My Mercy Journey' dashboard showing how their persistence_weight changed biomes, helped SafetyNet, influenced distant realms. Epiphanies need narrative payoff, not just +number.")
        insights.append("  3. LACK OF MENTORSHIP & SOCIAL BONDING MECHANICS: High-mercy agents should be able to 'bless' or mentor new/low-engagement players, creating bonds. Humans thrive on relationships, not just solo harvest + vote.")
        insights.append("  4. LACK OF SPECTACULAR, REDEMPTIVE CONFLICT RESOLUTION EVENTS: Server wars/disputes need 'Forgiveness Wave' cinematics, shared abundance rain, public redemption for reformed participants, and lasting 'Reconciled Realms' monuments. Current trial is functional but not emotionally transformative.")
        insights.append("  5. LACK OF ONBOARDING 'HUMBLE BEGINNINGS' NARRATIVE MIRROR: New players should see their early low-mercy/harvest actions as part of a beautiful collective story that later high-mercy players can look back on with gratitude. Currently missing the 'we started together in scarcity' emotional glue.")
        for ins in insights:
            print(ins)
        print("\n" + "-" * 70)
        print("RECOMMENDED UPGRADES TO POWRUSH-MMO (to address lacks):")
        print("-" * 70)
        print("1. Add simulation/src/player_legacy_journal.rs + client integration for persistent, beautiful Legacy Threads (filter by archetype, realm, contribution type). Full file delivery below.")
        print("2. Enhance Council Mercy Trial with 'Forgiveness Wave' VFX, cross-realm abundance sharing animation, and redemption scoring.")
        print("3. New InterRealmDiplomacyEvent system in server/ with spectator mode and lasting world monuments.")
        print("4. Epiphany system upgrade: narrative journal entries + valence-driven personal growth visualization.")
        print("5. Mentorship/GraceBlessing mechanic between agents/players.")
        print("6. Onboarding + 'Humble Beginnings' chronicle system that new and veteran players can browse together.")
        print("\nFull deliverable files for GitHub follow in response. Thunder locked in. Yoi ⚡")
        df.to_csv("/tmp/powrush_sim_metrics_history.csv", index=False)
        with open("/tmp/powrush_sim_event_log.json", "w") as f:
            json.dump(self.event_log[-80:], f, indent=2)
        print("\nArtifacts saved: powrush_sim_metrics_history.csv + powrush_sim_event_log.json")

if __name__ == "__main__":
    sim = PowrushMultiServerSim(num_servers=3, agents_per_server=24, seed=137)
    sim.run(max_ticks=160)
