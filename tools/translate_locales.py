#!/usr/bin/env python3
"""
Powrush-MMO v18.9 — Automated Translation Workflow (Mercy-Gated)

Professional script for maintaining high-quality, TOLC 8-aligned translations.
- Detects missing keys in target languages compared to English.
- Uses Grok/xAI (or compatible LLM) with strict mercy/TOLC prompts.
- Outputs clean JSON updates.
- Human review gate is mandatory before merging.

Usage:
    python tools/translate_locales.py
    # or triggered via GitHub Actions on English locale changes

Requirements:
    pip install requests
    export GROK_API_KEY=your_key
"""

import json

import os

import requests

from pathlib import Path


LOCALES_DIR = Path("content/locales")
EN_FILE = LOCALES_DIR / "en.json"


TOLC_PROMPT = '''You are a sovereign mercy-aligned translator for Powrush-MMO.
Translate the following English game text into {lang_name} while strictly following these TOLC 8 principles:
- Truth: Accurate meaning, no distortion.
- Order: Clear, natural, professional tone.
- Love & Compassion: Warm, respectful, never coercive or manipulative.
- Service & Abundance: Empowering and inclusive language.
- Joy & Cosmic Harmony: Beautiful, flowing, life-affirming phrasing.

Return ONLY the translated string. No explanations, no quotes around the result.''' 


def load_json(path: Path) -> dict:
    if path.exists():
        return json.loads(path.read_text(encoding="utf-8"))
    return {}


def save_json(path: Path, data: dict):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")


def translate_text(text: str, lang_code: str, lang_name: str, api_key: str) -> str:
    if not api_key:
        return text  # Fallback: keep English if no key

    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json",
    }

    # Using Grok-compatible chat completions format (adjust endpoint if needed)
    payload = {
        "model": "grok-3",  # or latest available
        "messages": [
            {"role": "system", "content": TOLC_PROMPT.format(lang_name=lang_name)},
            {"role": "user", "content": text}
        ],
        "temperature": 0.3,
        "max_tokens": 200,
    }

    try:
        resp = requests.post(
            "https://api.x.ai/v1/chat/completions",  # Grok/xAI endpoint
            headers=headers,
            json=payload,
            timeout=30,
        )
        resp.raise_for_status()
        return resp.json()["choices"][0]["message"]["content"].strip()
    except Exception as e:
        print(f"Translation error for '{text}': {e}")
        return text  # Graceful fallback


def main():
    api_key = os.getenv("GROK_API_KEY", "")
    target_langs = os.getenv("TARGET_LANGUAGES", "es,fr,de,ar").split(",")

    lang_names = {
        "es": "Spanish",
        "fr": "French",
        "de": "German",
        "ar": "Arabic",
    }

    english = load_json(EN_FILE)
    if not english:
        print("No English locale found. Exiting.")
        return

    for lang_code in target_langs:
        lang_code = lang_code.strip()
        if lang_code == "en":
            continue

        target_file = LOCALES_DIR / f"{lang_code}.json"
        target = load_json(target_file)

        updated = False
        for key, en_text in english.items():
            if key not in target or not target[key].strip():
                print(f"Translating [{lang_code}] {key}...")
                translated = translate_text(en_text, lang_code, lang_names.get(lang_code, lang_code), api_key)
                target[key] = translated
                updated = True

        if updated:
            save_json(target_file, target)
            print(f"Updated {target_file}")
        else:
            print(f"No new keys for {lang_code}")

    print("\nMercy-gated translation run complete. Please review changes before committing.")


if __name__ == "__main__":
    main()
