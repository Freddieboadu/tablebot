/**
 * commands/addteam.js
 * /addteam <team_name> — Add a new team to the league table with 0 stats.
 * Requires MANAGE_GUILD permission.
 */

const { SlashCommandBuilder, EmbedBuilder, PermissionFlagsBits } = require('discord.js');
const { readTable, writeTable, sortAndRecalculate } = require('../utils/tableUtils');
const { validateTeamName, teamExists } = require('../utils/validator');
const { pushHistory } = require('../utils/history');

module.exports = {
  data: new SlashCommandBuilder()
    .setName('addteam')
    .setDescription('Add a new team to the league table (Admin only)')
    .setDefaultMemberPermissions(PermissionFlagsBits.ManageGuild)
    .addStringOption(opt =>
      opt.setName('team_name')
        .setDescription('Name of the new team')
        .setRequired(true)),

  async execute(interaction) {
    const teamName = interaction.options.getString('team_name').trim().toUpperCase();

    // Validate name
    const nameCheck = validateTeamName(teamName);
    if (!nameCheck.valid) {
      return interaction.reply({ content: `❌ ${nameCheck.error}`, ephemeral: true });
    }

    const table = readTable();

    // Check for duplicates (case-insensitive)
    if (teamExists(table, teamName)) {
      return interaction.reply({
        content: `❌ Team **${teamName}** already exists in the table.`,
        ephemeral: true,
      });
    }

    // Save state to history before adding
    pushHistory(table, `Added team: ${teamName}`);

    // Create new team with zeroed stats
    const newTeam = {
      pos: table.length + 1,
      club: teamName,
      pl: 0,
      w: 0,
      d: 0,
      l: 0,
      gd: 0,
      pts: 0,
    };

    table.push(newTeam);
    const sorted = sortAndRecalculate(table);
    writeTable(sorted);

    // Find the new team's position after sorting
    const addedTeam = sorted.find(t => t.club === teamName);

    const embed = new EmbedBuilder()
      .setTitle('✅ Team Added')
      .setColor(0x00CC44) // Green
      .addFields(
        { name: 'Team', value: teamName, inline: true },
        { name: 'Starting Position', value: `#${addedTeam.pos}`, inline: true },
        { name: 'Added by', value: `<@${interaction.user.id}>`, inline: true }
      )
      .setFooter({ text: 'PBL — Pro Bot League | Use /table to view the full table' })
      .setTimestamp();

    return interaction.reply({ embeds: [embed] });
  },
};
