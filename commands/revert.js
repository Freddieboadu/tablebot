/**
 * commands/revert.js
 * /revert — Undo the last /update command.
 * Requires MANAGE_GUILD permission.
 */

const { SlashCommandBuilder, EmbedBuilder, PermissionFlagsBits } = require('discord.js');
const { writeTable, sortAndRecalculate, formatTable } = require('../utils/tableUtils');
const { popHistory, historyLength } = require('../utils/history');

module.exports = {
  data: new SlashCommandBuilder()
    .setName('revert')
    .setDescription('Undo the last /update command (Admin only)')
    .setDefaultMemberPermissions(PermissionFlagsBits.ManageGuild),

  async execute(interaction) {
    // Check remaining history count before popping
    const remaining = historyLength();
    if (remaining === 0) {
      return interaction.reply({
        content: '❌ No history available to revert. The table is at its original state.',
        ephemeral: true,
      });
    }

    // Pop the last snapshot
    const entry = popHistory();
    if (!entry) {
      return interaction.reply({
        content: '❌ No history available to revert.',
        ephemeral: true,
      });
    }

    // Restore the saved table snapshot
    const restored = sortAndRecalculate(entry.table);
    writeTable(restored);

    const tableString = formatTable(restored);
    const embed = new EmbedBuilder()
      .setTitle('↩️ Revert Successful')
      .setColor(0xFFA500) // Orange
      .addFields(
        {
          name: 'Reverted Result',
          value: `**${entry.description}**`,
          inline: false,
        },
        {
          name: 'Recorded at',
          value: `<t:${Math.floor(new Date(entry.timestamp).getTime() / 1000)}:F>`,
          inline: true,
        },
        {
          name: 'Reverted by',
          value: `<@${interaction.user.id}>`,
          inline: true,
        },
        {
          name: 'History entries remaining',
          value: `${remaining - 1}`,
          inline: true,
        }
      )
      .setFooter({ text: 'PBL — Pro Bot League' })
      .setTimestamp();

    const tableEmbed = new EmbedBuilder()
      .setTitle('🏆 Restored PBL League Table')
      .setDescription(tableString)
      .setColor(0xFFD700)
      .setFooter({ text: 'PBL — Pro Bot League | Sorted by PTS → GD → W' });

    return interaction.reply({ embeds: [embed, tableEmbed] });
  },
};
