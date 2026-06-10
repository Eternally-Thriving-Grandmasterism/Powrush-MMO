#!/usr/bin/env python3
"""
Powrush-MMO v18.9 — Professional Automated Translation API Integration

High-quality, mercy-gated translation system with deep Grok/xAI integration.

Features:
- Terminology glossary for consistent game terms (RBE, Divine Whispers, Council, etc.)
- Batch translation support
- Robust error handling + retries
- TOLC 8 aligned prompts with context
- Dry-run mode for safe exploration
- Clear human review gate

This script powers the GitHub Actions workflow for automated i18n.
"""

import json
import os
import time
from pathlib import Path
from typing import Dict, List, Optional

import requests


LOCALES_DIR = Path("content/locales")
EN_FILE = LOCALES_DIR / "en.json"
GLOSSARY_FILE = LOCALES_DIR / "glossary.json"


class TranslationAPI:
    def __init__(self, api_key: str):
        self.api_key = api_key
        self.base_url = "https://api.x.ai/v1/chat/completions"
        self.model = "grok-3"  # Update as newer models become available
        self.max_retries = 3
        self.retry_delay = 1.5

    def translate(
        self,
        text: str,
        target_lang: str,
        lang_name: str,
        context: Optional[str] = None,
        glossary: Optional[Dict] = None,
    ) -> str:
        if not self.api_key:
            return text

        prompt = self._build_tolc_prompt(target_lang, lang_name, context, glossary)

        for attempt in range(self.max_retries):
            try:
                response = requests.post(
                    self.base_url,
                    headers={
                        "Authorization": f"Bearer {self.api_key}",
                        "Content-Type": "application/json",
                    },
                    json={
                        "model": self.model,
                        "messages": [
                            {"role": "system", "content": prompt},
                            {"role": "user", "content": text},
                        ],
                        "temperature": 0.25,
                        "max_tokens": 300,
                    },
                    timeout=45,
                )
                response.raise_for_status()
                content = response.json()["choices"][0]["message"]["content"].strip()
                return self._clean_translation(content)
            except Exception as e:
                print(f"  Attempt {attempt + 1} failed: {e}")
                if attempt < self.max_retries - 1:
                    time.sleep(self.retry_delay * (attempt + 1))
                else:
                    print("  All retries exhausted. Keeping original English.")
                    return text
        return text

    def _build_tolc_prompt(
        self,
        target_lang: str,
        lang_name: str,
        context: Optional[str],
        glossary: Optional[Dict],
    ) -> str:
        base = f"""You are a sovereign, mercy-aligned translator for Powrush-MMO.

Translate the following English game text into {lang_name} while strictly honoring the TOLC 8 Mercy Gates:
- Truth: Preserve exact meaning with no distortion or exaggeration.
- Order: Use clear, natural, professional, and elegant phrasing.
- Love & Compassion: Warm, respectful, empowering tone. Never coercive or manipulative.
- Service & Abundance: Inclusive language that serves all players.
- Joy & Cosmic Harmony: Beautiful, flowing, life-affirming expression.

Game context: This is for an immersive Resource-Based Economy simulation focused on personal growth, collective mercy, and planetary thriving."""

        if glossary:
            base += "\n\nImportant terminology (use these exact translations when appropriate):\n"
            for term, translation in glossary.items():
                base += f"- {term} → {translation}\n"

        if context:
            base += f"\n\nAdditional context for this translation: {context}"

        base += "\n\nReturn ONLY the translated text. No explanations, no quotes, no markdown."
        return base

    def _clean_translation(self, text: str) -> str:
        # Remove common LLM artifacts
        text = text.strip().strip('"').strip("`").strip()
        if text.startswith("Translation:"):
            text = text.split(":", 1)[1].strip()
        return text


def load_json(path: Path) -> dict:
    if path.exists():
        try:
            return json.loads(path.read_text(encoding="utf-8"))
        except Exception:
            return {}
    return {}

def save_json(path: Path, data: dict):
    path.write_text(
        json.dumps(data, ensure_ascii=False, indent=2, ensure_ascii=False),
        encoding="utf-8"
    )


def main():
    api_key = os.getenv("GROK_API_KEY", "")
    target_langs = [l.strip() for l in os.getenv("TARGET_LANGUAGES", "es,fr,de,ar").split(",")]
    dry_run = os.getenv("DRY_RUN", "false").lower() == "true"

    lang_names = {
        "es": "Spanish",
        "fr": "French",
        "de": "German",
        "ar": "Arabic",
    }

    glossary = load_json(GLOSSARY_FILE)
    english = load_json(EN_FILE)

    if not english:
        print("No English source found. Exiting.")
        return

    translator = TranslationAPI(api_key)

    print("\n=== Powrush-MMO Automated Translation (Mercy-Gated) ===\n")

    for lang_code in target_langs:
        if lang_code == "en":
            continue

        target_file = LOCALES_DIR / f"{lang_code}.json"
        target = load_json(target_file)

        new_keys = []
        for key, en_text in english.items():
            if key not in target or not str(target.get(key, "")).strip():
                new_keys.append((key, en_text))

        if not new_keys:
            print(f"[{lang_code}] No new keys to translate.")
            continue

        print(f"\n[{lang_code}] Translating {len(new_keys)} new keys...")

        for key, en_text in new_keys:
            print(f"  → {key}")
            if dry_run:
                translated = f"[DRY RUN] {en_text}"
            else:
                translated = translator.translate(
                    en_text,
                    lang_code,
                    lang_names.get(lang_code, lang_code),
                    context="Part of Powrush-MMO onboarding and Divine Whispers system.",
                    glossary=glossary,
                )
            target[key] = translated

        if not dry_run:
            save_json(target_file, target)
            print(f"  Saved updates to {target_file}")
        else:
            print("  Dry run complete - no files modified.")

    print("\n=== Translation run complete. Human mercy review required before merge. ===\n")


if __name__ == "__main__":
    main()
