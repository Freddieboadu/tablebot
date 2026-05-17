/**
 * history.js
 * Push/pop helpers for the revert history stack stored in data/history.json.
 * The history file stores an array of snapshots (each snapshot is a full table array).
 * Maximum 20 entries are kept to limit storage.
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

const HISTORY_PATH = path.join(__dirname, '..', 'data', 'history.json');
const MAX_HISTORY = 20;

/**
 * Read the history stack from disk.
 * Returns an empty array if the file is missing or corrupt.
 * @returns {Array} Array of history entry objects
 */
function readHistory() {
  try {
    const raw = fs.readFileSync(HISTORY_PATH, 'utf8');
    return JSON.parse(raw);
  } catch {
    return [];
  }
}

/**
 * Atomically write the history stack to disk.
 * @param {Array} history
 */
function writeHistory(history) {
  const tmpPath = path.join(os.tmpdir(), `history_${Date.now()}.json.tmp`);
  fs.writeFileSync(tmpPath, JSON.stringify(history, null, 2), 'utf8');
  fs.renameSync(tmpPath, HISTORY_PATH);
}

/**
 * Push a new snapshot onto the history stack.
 * Enforces a maximum of MAX_HISTORY entries (oldest entries are dropped).
 * @param {Array} tableSnapshot - Deep copy of the table before a change
 * @param {string} description - Human-readable description of what changed
 */
function pushHistory(tableSnapshot, description) {
  const history = readHistory();
  history.push({
    table: JSON.parse(JSON.stringify(tableSnapshot)), // deep clone
    description,
    timestamp: new Date().toISOString(),
  });
  // Trim to the last MAX_HISTORY entries
  if (history.length > MAX_HISTORY) {
    history.splice(0, history.length - MAX_HISTORY);
  }
  writeHistory(history);
}

/**
 * Pop the most recent snapshot from the history stack.
 * @returns {{ table: Array, description: string, timestamp: string }|null}
 *   The last history entry, or null if history is empty.
 */
function popHistory() {
  const history = readHistory();
  if (history.length === 0) return null;
  const entry = history.pop();
  writeHistory(history);
  return entry;
}

/**
 * Return the number of entries currently in history.
 * @returns {number}
 */
function historyLength() {
  return readHistory().length;
}

module.exports = {
  pushHistory,
  popHistory,
  historyLength,
  readHistory,
};
