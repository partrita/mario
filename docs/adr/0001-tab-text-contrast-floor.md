# 1. A contrast floor for tab strip text

Date: 2026-11-11

Status: Accepted

## Context

A tab in the strip has two colours: the fill behind its title, and the title
itself. They come from different places, and nothing makes them agree.

The fills are derived at draw time from the window background's luminance
(`island_fills`, `frontends/marioterm/src/renderer/island.rs`). The title
colours come from `[colors] tabs` and `[colors] tabs-active`, which are
documented as *title* colours and are written into every generated theme
file.

On both palettes Mario ships, those two values collide:

| Palette | `tabs` | title luminance | inactive fill luminance | ratio |
| --- | --- | --- | --- | --- |
| `rose-pine-dawn` (light) | `#F2E9DE` | 0.825 | 0.811 | 1.02:1 |
| `rose-pine-moon` (dark) | `#2A273F` | 0.023 | 0.027 | 1.05:1 |

A ratio of 1:1 means the title is drawn in the fill's own colour. The tab
has a name, and the name cannot be seen — the reported bug, and the reason
this decision exists. Both palettes fail, not just the light one; each
palette's `foreground` (`#575279` / `#E0DEF4`) reads at 5.97:1 and 10.4:1,
so the palettes already contain a colour that works, and it is the one
their authors checked against the background.

Tab state is carried by more than the title colour: the fill steps between
inactive (`black@5%` / `white@5%`) and active (`white@92%` / `white@18%`),
the `!` mark plus its amber underline, and the 300 ms completion flash.

## Decision

Compute strip text colours at draw time, in `island.rs`, against the fill
the text actually sits on — the island fill composited over the window
background — and hold every run to a contrast floor.

1. **The floor is WCAG AA for small text, 4.5:1**, measured with WCAG
   relative luminance. A 12px tab title is small text; below the floor a
   glyph is technically drawn and practically not there.
2. **A colour that already clears the floor is left exactly as it is.**
   The rule adjusts, it does not restyle.
3. **Resolution order**: the configured colour, then the theme's
   `[colors] foreground`, then black or white. Black and white meet at
   4.58:1, so the last resort cannot fail on any background.
4. **The floor is absolute, and beats the colour hierarchy.** Where the
   configured pair collides, active and inactive titles converge on the
   same readable colour; which tab is which is then carried by the fill
   step, the mark and the flash. An unreadable tab is a tab the user
   cannot find, and no amount of hierarchy is worth that.
5. **The completion mark keeps its hue.** "A command finished" is carried
   by amber being neither the theme accent nor plain text, so the mark is
   scaled in brightness until it clears the floor (darkened first, since
   the fills that defeat amber are the light ones) rather than replaced by
   black or white. The underline uses the same adjusted colour.
6. **The position prefix keeps no dimming of its own.** It was drawn at
   55% alpha of the title colour, which lands at 2.36:1 on the shipped
   light fill and 4.25:1 on the dark one — both under the floor the prefix
   is now subject to. It draws in the title's colour and is held apart by
   the existing gap.
7. **The close button is floored on its base colour, and keeps its
   alphas.** It was drawn in the tab text colour, so on a broken theme it
   was as invisible as the title; the 55%-idle / 95%-hover alphas stay,
   because a cross that stays quiet until hovered is a deliberate
   affordance and the cross is a stroke, not text.
8. **Explicitly set colours are not exempt.** `[colors] tabs`,
   `tabs-active` and `color-automation` colours all go through the same
   rule; `[navigation] tab-text-contrast = "off"` is the escape hatch and
   defaults to `"auto"`.

## Consequences

The shipped palettes resolve to their own `foreground` colour, which is
what their authors intended; a third-party theme with an unreadable pair is
repaired without its author touching it, and a theme that already reads
correctly is untouched, pixel for pixel.

`[colors] tabs` and `tabs-active` change meaning: they are now a *first
choice* rather than the final word, which is why the man page entry for
them says so and `tab-text-contrast` exists. A user who wants a specific
colour drawn regardless must opt out.

The cost is a per-frame contrast computation per tab when `auto` is on
(memoised to the tabs that draw a mark or a flash, which is the only case
that iterates), and two colours that were previously distinct can now be
equal.

## Alternatives considered

- **Fix the shipped palette values instead.** Two lines, no runtime cost —
  and it leaves every existing user's generated theme file (never
  overwritten after first run), every third-party theme, and every hand-set
  `[colors] tabs` exactly as broken as it is now. The report asked for
  readability "regardless of theme change".
- **Let an explicitly configured colour win.** Requires knowing whether a
  value came from a theme file or from the built-in default, which
  `#[serde(default = ...)]` erases; it would mean making every `Colors`
  field an `Option` and changing theme round-tripping. It also mainly
  preserves configurations nobody wants.
- **Keep the hierarchy by dimming the inactive title to just above the
  floor.** The floor and the hierarchy then fight over the same knob, and
  the loser is legibility on whichever palette has less headroom.
- **Carry the hierarchy with weight or size instead of colour.** The UI
  text layer resolves one font per run, so a run cannot be re-weighted;
  this is the constraint that also forces the mark and the index to be
  separate draws.
- **Change the fills instead of the text.** The fills are what make the
  strip adapt to the window background, including under OSC 11 and
  transparency; the collision is in the text colours, so that is where the
  fix belongs.

## Out of scope

Deliberately not covered, and worth revisiting on their own merits: the
progress bar under the strip (a configurable colour drawn on the window
background, not on a tab fill) and the colour picker panel (its own
surface). Neither is strip text and neither was part of the reported bug.
