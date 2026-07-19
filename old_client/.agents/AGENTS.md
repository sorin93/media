# Workspace Rules

## Environment & Command Execution
- Do not run build, installation, or compilation commands (such as `npm run build`, `npx vite`, `npm install`) as these toolchains are not available in the agent's sandbox. Rely on the user to run builds and report any compilation or lint errors.

## Project Guidelines
- Use JavaScript only (no TypeScript).
- Svelte 5 with Runes only.
- Component Layout: HTML at the top, then `<style>`, then `<script>`.
- Folder Layout: Domain-based folders under `src/` (e.g. `src/User/`, `src/Media/`).
- Shared State: `src/lib/store.svelte.js`.
- API Fetching: `src/lib/fetchApi.svelte.js`.
- Routing: `src/Router`.

## Coding Style
- Use arrow functions where possible: `const fn = arg => expr;` (avoid brackets for single parameters; omit braces/return for single expressions).
- Add short HTML comments `<!-- Label -->` above important HTML sections.
- Minimize CSS/JS comments and blank lines.
