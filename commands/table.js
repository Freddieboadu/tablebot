/**
 * commands/table.js
 * /table — Display the full league table as a Discord embed.
 */

const { SlashCommandBuilder, EmbedBuilder } = require('discord.js');
const { readTable, sortAndRecalculate, formatTable } = require('../utils/tableUtils');

module.exports = {
  data: new SlashCommandBuilder()
    .setName('table')
    .setDescription('Display the current PBL league table'),

  async execute(interaction) {
    // Read and sort the table
    const raw = readTable();
    const table = sortAndRecalculate(raw);

    if (table.length === 0) {
      return interaction.reply({
        content: '⚠️ The league table is currently empty. Use `/addteam` to add teams.',
        ephemeral: true,
      });
    }

    const tableString = formatTable(table);

    const embed = new EmbedBuilder()
      .setTitle('🏆 PBL League Table')
      .setDescription(tableString)
      .setColor(0xFFD700) // Gold
      .setFooter({ text: 'PBL — Pro Bot League | Sorted by PTS → GD → W' })
      .setTimestamp();

    return interaction.reply({ embeds: [embed] });
  },
};
