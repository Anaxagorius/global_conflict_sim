# Kaleb’s Modern World: Global Crisis Command

A modern-era grand strategy game concept focused on fun, clarity, and modern geopolitical decision-making.

## Executive Summary

- **Project title:** Kaleb’s Modern World: Global Crisis Command (working title)
- **Genre:** Modern grand strategy / global conflict game
- **Audience:** Strategy-minded players (teen to adult), with strong accessibility and replayability
- **Vision:** Put players in control of a modern nation balancing diplomacy, economics, technology, and high-impact conflict on a reactive Earth
- **Tech direction:** Self-contained desktop app using Rust + CesiumJS

## Core Design Philosophy

**Global strategy, not spreadsheet warfare.**

The game should:

- Feel intuitive and playable, not overwhelming
- Reward strategic thinking over memorization
- Represent modern realities (sanctions, cyber operations, media influence, alliances)
- Teach geopolitics through play

Inspirational blend:

- Hearts of Iron scope
- Civilization clarity
- DEFCON tension
- Modern geopolitics complexity

## Core Gameplay Pillars

### 1) World Map (Primary Experience)

- Toggle between:
  - Flat strategic map
  - 3D rotatable globe
- Powered by CesiumJS for smooth global-to-regional zoom
- Regions are abstracted strategic areas (population, industry, infrastructure), not raw province-level simulation

### 2) Politics & Governance

- Regime types:
  - Democracy
  - Authoritarian
  - Hybrid / transitional
- Internal stability systems:
  - Public approval
  - Protest risk
  - Elite factions (military, business, media)
- Elections, coups, and reforms are event-driven (not constant micromanagement)

### 3) Diplomacy & Influence

- Relationship scores between nations
- Influence actions:
  - Aid
  - Pressure
  - Treaties
  - Covert support
- Alliance blocs and proxy conflicts preferred over constant direct total war

### 4) Economy, Trade & Sanctions

- Simplified but meaningful resource economy:
  - Energy
  - Industry
  - Technology
- Trade routes and chokepoints matter strategically
- Sanctions and economic pressure can drive medium-term political instability

### 5) Military & Conflict

- Modern force model: fewer units, higher cost, higher impact
- Action types:
  - Limited wars
  - Precision strikes
  - Peacekeeping operations
- Nuclear systems primarily function as deterrence, not straightforward win conditions

### 6) Information & Cyber Warfare

First-class gameplay system including:

- Disinformation campaigns
- Election interference
- Media control
- Cyber operations that affect stability, economy, and military readiness

## Technical Architecture

### Backend / Core Logic: Rust

- Deterministic turn/phase resolution
- High performance and memory safety
- Strong long-term maintainability for a solo-friendly project

### Rendering: CesiumJS

- Supports both immersive globe and strategic flat presentations
- Web-based rendering embedded in desktop shell
- Clear separation between logic (Rust) and presentation (JavaScript)

### Application Shell

- Desktop app using Tauri (or similar lightweight shell)
- Rust backend + web frontend
- Offline-first and self-contained

### Data & Modularity

- JSON / binary game data for countries, regions, and events
- Designed to be extensible and mod-friendly over time

## User Experience Goals

- Clean operations-room style UI
- Dark mode default
- Readable metrics and iconography
- Rich tooltip guidance
- Accessibility features:
  - Optional advisor hints
  - Difficulty sliders
  - Pause-and-think pacing (no twitch requirements)

## Educational and Personal Value

This project is intended to build:

- Systems thinking
- World geography and geopolitical awareness
- Understanding of economics and consequence chains
- Practical software and game development skills

## Why This Project Works

- Modern, relevant subject matter
- Clear scope: game first, simulation second
- Built on powerful open technologies
- Scales from accessible to deep strategy
- Strong fit for motivated solo development

## Closing

This project aims to deliver a memorable, modern global strategy experience centered on experimentation, consequence, and learning-through-play—built to be both meaningful and fun.
