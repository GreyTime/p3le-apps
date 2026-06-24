# Lernplan: Goal Tracker in Rust

Ein selbstgebauter, gamified Goal Tracker im bestehenden p3le-apps Workspace
(Yew Frontend + Rocket Backend). Ziel: **Rust verstehen**, nicht abschreiben.

---

## 0. Wie du diesen Plan nutzt

- Arite phase nacheinander ab. Spring nicht vor.
- Für jede Phase gilt: **erst selbst probieren**, dann Doku/Beispiele lesen,
  erst als letztes KI gezielt für eine konkrete Frage nutzen.
- Schreibe nach jeder Phase kurz (2–3 Sätze) auf, was du verstanden hast und
  was unklar blieb. Das ist dein Lerntagebuch.
- Wenn du hängst: formuliere die Frage in eigenen Worten, bevor du fragst.
  Oft löst sich das Problem beim Formulieren.

---

## Phase 1 — Rust Grundlagen (ohne Web)

Lernziel: du kannst ein eigenes kleines Konsolenprogramm schreiben, das
Datenstrukturen definiert, mit `enum`/`struct` arbeitet und Ein-/Ausgabe macht.

Aufgaben (jeweils in einer eigenen Datei, z. B. `backend/src/bin/learn_*.rs`):

1. **`hello`**: `fn main`, `println!`, `String`, `&str`, ownership beim
   Übergeben einer `String` an eine Funktion.
2. **`struct_goal`**: Definiere ein `struct Goal { title, daily_action, ... }`
   und lege einen `Vec<Goal>` an. Iteriere und gib sie aus.
3. **`enum_status`**: Ein `enum DayStatus { Done, Skipped, Pending }` und eine
   Funktion, die zu einem Status einen Text liefert (`match`).
4. **`option_result`**: Lies eine Zahl von stdin (`std::io`), parse mit
   `str::parse`, behandle Fehler mit `Option`/`Result` — kein `unwrap` im
   finalen Code.
5. **`serde_io`**: Speichere/lade einen `Vec<Goal>` als JSON aus einer Datei
   (Crate `serde` + `serde_json`). Markierung mit `#[derive(...)]`.

Checkpoint: Du kannst erklären — ownership, borrow, `Option`, `Result`, `enum`,
`match`, Trait (`Debug`, `Serialize`). Wenn nicht → nochmal.

---

## Phase 2 — Yew verstehen (am bestehenden Frontend)

Lernziel: du verstehst, wie die bestehenden Seiten funktionieren, und kannst
eine eigene leere Seite einhängen.

Aufgaben:

1. Lies `frontend/src/main.rs` und alle Dateien unter `src/pages/`. Erkläre
   dir selbst: was macht `#[component]`, was ist `Html`, was macht `Switch`?
2. Füge eine neue Route `GoalTracker` hinzu und eine leere Seite unter
   `src/pages/goal_tracker/mod.rs` (analog zu `habit_tracker`).
3. Trage sie in `pages/mod.rs`, in `Route` und in `switch` ein und verlinke
   sie auf der `Home`-Seite.
4. Lass die Seite eine Überschrift und einen Button rendern. Baue sie mit
   `trunk serve` (Frontend) und prüfe, ob sie erreichbar ist.

Checkpoint: Seite ist im Browser sichtbar und per Nav erreichbar.

---

## Phase 3 — State & User Interaction (Frontend-only)

Lernziel: du kannst mit `use_state` / `use_state_eq` Daten im Frontend halten
und an Events binden.

Aufgaben (alles noch ohne Backend, Daten nur im Frontend):

1. Ein Eingabefeld für den Goal-Titel und eins für die "tägliche Aktion".
2. Ein "Hinzufügen"-Button, der den Goal in eine Liste (`Vec<Goal>`) im
   State packt und die Liste darstellt.
3. Jeder Goal-Eintrag zeigt: Titel, Aktion und einen "heute erledigt"-Button.
4. Klick auf erledigt toggelt den Status für heute.
5. Speichere den State in `localStorage` (über `web-sys` oder `gloo-storage`)
   damit ein Reload die Daten behält.

Lernthemen: `use_state`, Closures in Yew, `Callback`, `MouseEvent`, `web-sys`
oder `gloo`. Lies die Yew-Doku zu Hooks, bevor du tippst.

Checkpoint: Du kannst Goals anlegen, als erledigt markieren, Reload überlebt.

---

## Phase 4 — Gamification (Frontend-only)

Lernziel: du modellierst Logik in Rust-Funktionen und testest sie.

Ideen (selbst auswählen / eigene dazu):

- **Streak**: wie viele Tage am Stück erledigt? Anzeige + kleines Feuer-Emoji.
- **XP / Level**: pro erledigten Tag +10 XP, alle 100 XP ein Level.
- **Quest-Log**: tägliche Aktion als "Quest" mit Status-Icon.
- **Belohnungen**: bei bestimmten Levels freischaltbare Badges.

Aufgaben:

1. Schreibe die Logik als reine Funktionen in einem Modul
   `goal_tracker/logic.rs` (kein HTML dort). Z. B. `fn streak(entries: &[Entry]) -> u32`.
2. Teste diese Funktionen mit `#[cfg(test)] mod tests` und `cargo test -p frontend`.
3. Im Component rufst du nur die Funktionen auf und renderst die Ergebnisse.

Checkpoint: Logik hat grüne Tests, UI zeigt Streak/XP.

---

## Phase 5 — Backend & Persistenz (Rocket)

Lernziel: du verstehst HTTP-Handler, JSON-(De)Serialisierung und State in Rocket.

Aufgaben:

1. Definiere die Datentypen (`Goal`, `Entry`, ...) in einem Crate oder Modul,
   das beide (Frontend via `serde`, Backend) nutzen können — oder zunächst
   duplizieren und später refactoren.
2. Rocket-Endpoints:
   - `GET /api/goals` → alle Goals
   - `POST /api/goals` → neuen Goal anlegen
   - `POST /api/goals/:id/done` → heutigen Tag als erledigt markieren
   - `GET /api/goals/:id/entries` → Historie
3. Persistenz: erst in-memory (`Mutex<Vec<Goal>>` in `rocket::State`),
   später Datei/SQLite (`rusqlite` oder `sqlx`).
4. Frontend ruft das Backend ab (CORS, `gloo-net` oder `web-sys::fetch`).
   Tausche `localStorage` durch Backend-Aufrufe aus.

Lernthemen: `#[get/post]`, `Json<T>`, `State`, `Mutex`, `serde`, `gloo-net`.

Checkpoint: Goals liegen im Backend, reload holt Daten vom Server.

---

## Phase 6 — Polishing & eigene Erweiterungen

Jetzt bist du selbstständig. Wähle eigene Features:

- Belohnungs-Animations-Logik
- Wochen-/Monatsstatistik
- Mehrere Goals parallel
- Dark Mode
- Deployment (Trunk build → Rocket serviert `dist`, so wie es schon ist)

---

## Rust-Lernpfad parallel zu den Phasen

Arbeite diese Themen entlang der Phasen durch (Buch "The Rust Programming Language"
oder Rustlings):

| Phase | Thema |
|------|-------|
| 1 | Ownership, Borrowing, References, `String` vs `&str` |
| 1 | Structs, Enums, `match`, `Option`, `Result` |
| 1 | Traits, Generics, `derive` |
| 1 | Collections (`Vec`, `HashMap`), Iteratoren |
| 2 | Module, `pub`, `use`, Crates |
| 3 | Closures, Lifetimes (light) |
| 4 | Testing, `#[cfg(test)]` |
| 5 | Concurrency (`Mutex`, `Arc`), Error-Handling |
| 5 | Serde, Rocket, Async (`async`/`await`) Basics |

---

## KI-Nutzung – Spielregeln für dich selbst

- Erlaubt: "erkläre Konzept X", "warum kompiliert das nicht", "wie teste ich Y".
- Verboten (für deinen Lernfortschritt): "schreib mir die Komponente".
- Wenn du Code von KI nimmst: erst Zeile für Zeile erklären, dann selbst
  abtippen (nicht kopieren), dann abwandeln.

---

## Lerntagebuch

Führe hier kurz mit, was pro Phase geklappt hat und was offen ist:

### Phase 1
- 

### Phase 2
- 

### Phase 3
- 

### Phase 4
- 

### Phase 5
- 

### Phase 6
- 
