> Historical design reference. The generated concept is retained locally and excluded from public source; it is not a shipped application asset. Current Restnaa visuals are code/SVG and local icons.

# M1 design reference

`m1-concept.png` is the Image Gen concept used for the M1 shell. It is a reference, not a shipped UI screenshot. The real interface uses code-native text, controls, local Lucide icons, and system fonts; no raster image is rendered as application UI.

Direction: white sidebar and surfaces; cool-gray `#f5f7f8` workspace; dark teal `#153c3a` text; `#246b60` accent; pale mint selected navigation; thin gray borders and restrained corners. Typography uses Segoe UI/system sans, 42px primary headings at 1200px and 56px at 1536px, 24–32px section headings, 16–20px body and navigation, 13–17px secondary labels. Shared panel padding scales from 22–30px to 28–40px with 14–16px gaps. A circular pause mark anchors the sidebar. Local outline icons use consistent 1.8px strokes.

Allowed dashboard copy is the concept's brand, Dashboard/Welcome/Settings/History navigation, Foundation preview, heading/promise, introduction/CTA, two reminder descriptions/availability notices, local device labels, and privacy footer. Runtime storage status replaces the concept's temporary “Checking…” text with a genuine backend result or visible error. Other requested views extend the same shell with honest foundation information.

Intentional implementation adaptations: native operating-system title bar supplied by Tauri rather than drawn in HTML; desktop minimum window 720×600; responsive browser layout at 390px; Lucide outline house icon instead of concept's filled house; no textured background or gradient. These keep the shell consistent, lightweight, and accessible. There are no timers, counters, charts, fake controls, or generated content in the shipped app.
