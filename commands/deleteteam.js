/**
 * commands/deleteteam.js
 * /deleteteam <team_name> — Remove a team from the league table with button confirmation.
 * Requires MANAGE_GUILD permission.
 */

const {
  SlashCommandBuilder,
  EmbedBuilder,
  ActionRowBuilder,
  ButtonBuilder,
  ButtonStyle,
  PermissionFlagsBits,
  ComponentType,
} = require('discord.js');
const { readTable, writeTable, findTeam, sortAndRecalculate } = require('../utils/tableUtils');
const { pushHistory } = require('../utils/history');

module.exports = {
  data: new SlashCommandBuilder()
    .setName('deleteteam')
    .setDescription('Remove a team from the league table (Admin only)')
    .setDefaultMemberPermissions(PermissionFlagsBits.ManageGuild)
    .addStringOption(opt =>
      opt.setName('team_name')
        .setDescription('Name of the team to remove')
        .setRequired(true)),

  async execute(interaction) {
    const teamNameInput = interaction.options.getString('team_name');
    const table = readTable();

    // Find the team (case-insensitive)
    const team = findTeam(table, teamNameInput);
    if (!team) {
      return interaction.reply({
        content: `❌ Team **${teamNameInput}** not found in the table. Check spelling.`,
        ephemeral: true,
      });
    }

    // Build confirmation prompt with Yes/No buttons
    const confirmId = `delete_confirm_${interaction.user.id}_${Date.now()}`;
    const cancelId = `delete_cancel_${interaction.user.id}_${Date.now()}`;

    const confirmButton = new ButtonBuilder()
      .setCustomId(confirmId)
      .setLabel('✅ Yes, Delete')
      .setStyle(ButtonStyle.Danger);

    const cancelButton = new ButtonBuilder()
      .setCustomId(cancelId)
      .setLabel('❌ Cancel')
      .setStyle(ButtonStyle.Secondary);

    const row = new ActionRowBuilder().addComponents(confirmButton, cancelButton);

    const warningEmbed = new EmbedBuilder()
      .setTitle('⚠️ Confirm Team Deletion')
      .setColor(0xFF4444) // Red
      .setDescription(
        `Are you sure you want to remove **${team.club}** from the league table?\n\n` +
        `> ⚠️ All match data for this team will **remain in the revert history** but the team will be removed from the current table.\n\n` +
        `This action can be undone with \`/revert\`.`
      )
      .addFields(
        { name: 'Team', value: team.club, inline: true },
        { name: 'Current Position', value: `#${team.pos}`, inline: true },
        { name: 'Points', value: String(team.pts), inline: true },
      )
      .setFooter({ text: 'This prompt will expire in 30 seconds' })
      .setTimestamp();

    const reply = await interaction.reply({
      embeds: [warningEmbed],
      components: [row],
      ephemeral: true,
    });

    // Collect a single button interaction within 30 seconds
    let collected;
    try {
      collected = await reply.awaitMessageComponent({
        filter: i => i.user.id === interaction.user.id,
        componentType: ComponentType.Button,
        time: 30_000,
      });
    } catch {
      // Timeout — disable buttons
      const disabledRow = new ActionRowBuilder().addComponents(
        ButtonBuilder.from(confirmButton).setDisabled(true),
        ButtonBuilder.from(cancelButton).setDisabled(true),
      );
      await interaction.editReply({
        content: '⏰ Confirmation timed out. No changes were made.',
        embeds: [],
        components: [disabledRow],
      });
      return;
    }

    if (collected.customId === cancelId) {
      await collected.update({
        content: '❌ Deletion cancelled. No changes were made.',
        embeds: [],
        components: [],
      });
      return;
    }

    // User confirmed — proceed with deletion
    // Save state to history
    pushHistory(table, `Deleted team: ${team.club}`);

    // Remove the team from the table
    const updatedTable = table.filter(t => t.club !== team.club);
    const sorted = sortAndRecalculate(updatedTable);
    writeTable(sorted);

    const successEmbed = new EmbedBuilder()
      .setTitle('🗑️ Team Deleted')
      .setColor(0xFF4444)
      .addFields(
        { name: 'Removed Team', value: team.club, inline: true },
        { name: 'Deleted by', value: `<@${interaction.user.id}>`, inline: true },
        { name: 'Teams remaining', value: String(sorted.length), inline: true }
      )
      .setDescription('The team has been removed from the current table.\nUse `/revert` to restore it if needed.')
      .setFooter({ text: 'PBL — Pro Bot League' })
      .setTimestamp();

    await collected.update({
      embeds: [successEmbed],
      components: [],
    });
  },
};
