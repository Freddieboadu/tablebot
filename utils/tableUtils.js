/**
 * tableUtils.js
 * Utility functions for sorting, recalculating positions, and formatting the league table.
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

const TABLE_PATH = path.join(__dirname, '..', 'data', 'table.json');

/**
 * Read the current table from disk.
 * If the file is missing or corrupt, returns an empty array.
 * @returns {Array} Array of team objects
 */
function readTable() {
  try {
    const raw = fs.readFileSync(TABLE_PATH, 'utf8');
    return JSON.parse(raw);
  } catch {
    // Gracefully handle missing or corrupt file by returning empty table
    return [];
  }
}

/**
 * Atomically write the table to disk.
 * Writes to a temp file first, then renames to avoid partial writes on crash.
 * @param {Array} table - Array of team objects
 */
function writeTable(table) {
  const tmpPath = path.join(os.tmpdir(), `table_${Date.now()}.json.tmp`);
  fs.writeFileSync(tmpPath, JSON.stringify(table, null, 2), 'utf8');
  fs.renameSync(tmpPath, TABLE_PATH);
}

/**
 * Sort the table by: PTS desc → GD desc → W desc → club name asc (alphabetical tiebreaker).
 * @param {Array} table
 * @returns {Array} Sorted copy of the table
 */
function sortTable(table) {
  return [...table].sort((a, b) => {
    if (b.pts !== a.pts) return b.pts - a.pts;
    if (b.gd !== a.gd) return b.gd - a.gd;
    if (b.w !== a.w) return b.w - a.w;
    return a.club.localeCompare(b.club);
  });
}

/**
 * Recalculate the 'pos' field for each team after sorting.
 * @param {Array} table - Already-sorted array
 * @returns {Array} Table with updated pos fields
 */
function recalculatePositions(table) {
  return table.map((team, index) => ({ ...team, pos: index + 1 }));
}

/**
 * Sort and recalculate positions in one step.
 * @param {Array} table
 * @returns {Array} Sorted table with correct positions
 */
function sortAndRecalculate(table) {
  return recalculatePositions(sortTable(table));
}

/**
 * Find a team in the table by name (case-insensitive, trimmed).
 * @param {Array} table
 * @param {string} name
 * @returns {Object|null} The team object or null if not found
 */
function findTeam(table, name) {
  const normalized = name.trim().toUpperCase();
  return table.find(t => t.club.toUpperCase() === normalized) || null;
}

/**
 * Column widths for the formatted table display.
 */
const COL_POS = 4;
const COL_CLUB = 16;
const COL_STAT = 3;
const COL_GD = 5;
const COL_PTS = 4;

/**
 * Format the league table as a Discord embed-friendly code block string.
 * Columns: POS | CLUB | PL | W | D | L | GD | PTS
 * @param {Array} table - Sorted table with positions assigned
 * @returns {string} Formatted table string (for embed description)
 */
function formatTable(table) {
  // Header
  const header = `\`\`\`\n${'POS'.padEnd(COL_POS)} ${'CLUB'.padEnd(COL_CLUB)} ${'PL'.padStart(COL_STAT)} ${'W'.padStart(COL_STAT)} ${'D'.padStart(COL_STAT)} ${'L'.padStart(COL_STAT)} ${'GD'.padStart(COL_GD)} ${'PTS'.padStart(COL_PTS)}\n`;
  const divider = `${'-'.repeat(45)}\n`;

  const rows = table.map(t => {
    const pos = t.pos === 1 ? '🏆 ' : `${String(t.pos).padEnd(2)} `;
    const club = t.club.padEnd(COL_CLUB);
    const pl = String(t.pl).padStart(COL_STAT);
    const w = String(t.w).padStart(COL_STAT);
    const d = String(t.d).padStart(COL_STAT);
    const l = String(t.l).padStart(COL_STAT);
    const gd = (t.gd >= 0 ? `+${t.gd}` : String(t.gd)).padStart(COL_GD);
    const pts = String(t.pts).padStart(COL_PTS);
    return `${pos}${club}${pl}${w}${d}${l}${gd}${pts}`;
  });

  return `${header}${divider}${rows.join('\n')}\n\`\`\``;
}

module.exports = {
  readTable,
  writeTable,
  sortTable,
  recalculatePositions,
  sortAndRecalculate,
  findTeam,
  formatTable,
};
