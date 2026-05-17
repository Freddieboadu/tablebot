/**
 * validator.js
 * Input validation helpers for the PBL TableBot.
 */

/**
 * Validate a score value: must be a non-negative integer.
 * @param {string|number} value
 * @returns {{ valid: boolean, value?: number, error?: string }}
 */
function validateScore(value) {
  const num = Number(value);
  if (!Number.isInteger(num) || num < 0) {
    return { valid: false, error: `Score "${value}" must be a non-negative whole number.` };
  }
  return { valid: true, value: num };
}

/**
 * Validate a team name: non-empty string, max 32 chars.
 * @param {string} name
 * @returns {{ valid: boolean, error?: string }}
 */
function validateTeamName(name) {
  if (!name || typeof name !== 'string') {
    return { valid: false, error: 'Team name must be a non-empty string.' };
  }
  const trimmed = name.trim();
  if (trimmed.length === 0) {
    return { valid: false, error: 'Team name cannot be blank.' };
  }
  if (trimmed.length > 32) {
    return { valid: false, error: 'Team name cannot exceed 32 characters.' };
  }
  return { valid: true };
}

/**
 * Check whether two team names are equal (case-insensitive).
 * @param {string} a
 * @param {string} b
 * @returns {boolean}
 */
function teamsEqual(a, b) {
  return a.trim().toUpperCase() === b.trim().toUpperCase();
}

/**
 * Check whether a team already exists in the table (case-insensitive).
 * @param {Array} table
 * @param {string} name
 * @returns {boolean}
 */
function teamExists(table, name) {
  const normalized = name.trim().toUpperCase();
  return table.some(t => t.club.toUpperCase() === normalized);
}

module.exports = {
  validateScore,
  validateTeamName,
  teamsEqual,
  teamExists,
};
