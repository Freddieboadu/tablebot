/**
 * commands/update.js
 * /update <home_team> <home_score> <away_team> <away_score>
 * Accept a match result and update both teams' stats.
 */

const { SlashCommandBuilder, EmbedBuilder } = require('discord.js');
const { readTable, writeTable, findTeam, sortAndRecalculate, formatTable } = require('../utils/tableUtils');
const { validateScore, teamsEqual } = require('../utils/validator');
const { pushHistory } = require('../utils/history');

module.exports = {
  data: new SlashCommandBuilder()
    .setName('update')
    .setDescription('Record a match result and update the league table')
    .addStringOption(opt =>
      opt.setName('home_team')
        .setDescription('Name of the home team')
        .setRequired(true))
    .addIntegerOption(opt =>
      opt.setName('home_score')
        .setDescription('Goals scored by the home team')
        .setRequired(true)
        .setMinValue(0))
    .addStringOption(opt =>
      opt.setName('away_team')
        .setDescription('Name of the away team')
        .setRequired(true))
    .addIntegerOption(opt =>
      opt.setName('away_score')
        .setDescription('Goals scored by the away team')
        .setRequired(true)
        .setMinValue(0))
    .addBooleanOption(opt =>
      opt.setName('force')
        .setDescription('Force update even if a duplicate fixture warning is raised')
        .setRequired(false)),

  async execute(interaction) {
    const homeTeamInput = interaction.options.getString('home_team');
    const homeScore = interaction.options.getInteger('home_score');
    const awayTeamInput = interaction.options.getString('away_team');
    const awayScore = interaction.options.getInteger('away_score');
    const force = interaction.options.getBoolean('force') ?? false;

    // Validate scores (Discord enforces min 0 via setMinValue, but double-check)
    const homeVal = validateScore(homeScore);
    const awayVal = validateScore(awayScore);
    if (!homeVal.valid) {
      return interaction.reply({ content: `❌ ${homeVal.error}`, ephemeral: true });
    }
    if (!awayVal.valid) {
      return interaction.reply({ content: `❌ ${awayVal.error}`, ephemeral: true });
    }

    // Team cannot play itself
    if (teamsEqual(homeTeamInput, awayTeamInput)) {
      return interaction.reply({
        content: '❌ A team cannot play against itself.',
        ephemeral: true,
      });
    }

    const table = readTable();

    // Find teams (case-insensitive)
    const homeTeam = findTeam(table, homeTeamInput);
    const awayTeam = findTeam(table, awayTeamInput);

    if (!homeTeam) {
      return interaction.reply({
        content: `❌ Team **${homeTeamInput}** not found. Check spelling or use \`/addteam\` to add it.`,
        ephemeral: true,
      });
    }
    if (!awayTeam) {
      return interaction.reply({
        content: `❌ Team **${awayTeamInput}** not found. Check spelling or use \`/addteam\` to add it.`,
        ephemeral: true,
      });
    }

    // Duplicate fixture detection: warn if both teams have the same PL count
    // (heuristic: if their game counts differ by less than 1, this might be a duplicate)
    // We store a simple fixture log — for now, warn based on the force flag
    if (!force) {
      // Check if any recent history entry describes the exact same fixture
      const { readHistory } = require('../utils/history');
      const history = readHistory();
      const dupDesc = `${homeTeam.club} ${homeScore}-${awayScore} ${awayTeam.club}`;
      const isDuplicate = history.some(h => h.description === dupDesc);
      if (isDuplicate) {
        return interaction.reply({
          content: `⚠️ This exact result (**${dupDesc}**) appears to have already been recorded. Use \`/update\` with the \`force:True\` option to override.`,
          ephemeral: true,
        });
      }
    }

    // Save current state to history before making changes
    const description = `${homeTeam.club} ${homeScore}-${awayScore} ${awayTeam.club}`;
    pushHistory(table, description);

    // Determine outcomes
    let homeWins = 0, homeDraws = 0, homeLosses = 0;
    let awayWins = 0, awayDraws = 0, awayLosses = 0;
    let homePoints = 0, awayPoints = 0;
    let resultLabel = '';

    if (homeScore > awayScore) {
      homeWins = 1; homeLosses = 0; homeDraws = 0; homePoints = 3;
      awayLosses = 1; awayPoints = 0;
      resultLabel = `🏠 **${homeTeam.club}** wins!`;
    } else if (homeScore < awayScore) {
      awayWins = 1; awayPoints = 3;
      homeLosses = 1; homePoints = 0;
      resultLabel = `✈️ **${awayTeam.club}** wins!`;
    } else {
      homeDraws = 1; homePoints = 1;
      awayDraws = 1; awayPoints = 1;
      resultLabel = `🤝 Draw!`;
    }

    const homeGD = homeScore - awayScore;
    const awayGD = awayScore - homeScore;

    // Apply updates to home team
    homeTeam.pl += 1;
    homeTeam.w += homeWins;
    homeTeam.d += homeDraws;
    homeTeam.l += homeLosses;
    homeTeam.gd += homeGD;
    homeTeam.pts += homePoints;

    // Apply updates to away team
    awayTeam.pl += 1;
    awayTeam.w += awayWins;
    awayTeam.d += awayDraws;
    awayTeam.l += awayLosses;
    awayTeam.gd += awayGD;
    awayTeam.pts += awayPoints;

    // Sort and recalculate positions
    const sorted = sortAndRecalculate(table);
    writeTable(sorted);

    // Build confirmation embed
    const tableString = formatTable(sorted);
    const embed = new EmbedBuilder()
      .setTitle('⚽ Match Result Recorded')
      .setColor(0x1E90FF) // Blue
      .addFields(
        {
          name: 'Result',
          value: `**${homeTeam.club}** ${homeScore} – ${awayScore} **${awayTeam.club}**\n${resultLabel}`,
          inline: false,
        },
        {
          name: 'Recorded by',
          value: `<@${interaction.user.id}>`,
          inline: true,
        }
      )
      .setFooter({ text: 'Use /revert to undo this change' })
      .setTimestamp();

    // Append updated table
    const tableEmbed = new EmbedBuilder()
      .setTitle('🏆 Updated PBL League Table')
      .setDescription(tableString)
      .setColor(0xFFD700)
      .setFooter({ text: 'PBL — Pro Bot League | Sorted by PTS → GD → W' });

    return interaction.reply({ embeds: [embed, tableEmbed] });
  },
};
