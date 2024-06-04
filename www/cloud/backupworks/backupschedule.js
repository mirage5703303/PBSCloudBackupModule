Ext.define('Proxmox.cloud.BackupJob', {
    extend: 'Ext.panel.Panel',
    alias: 'widget.cloudbackupjob',

    requires: [
        'Proxmox.grid.BackupJobGrid',
        'Proxmox.form.field.BackupJobField'
    ],

    layout: 'fit',
    border: false,

    initComponent: function() {
        var me = this;

        me.items = [{
            xtype: 'backupjobgrid',
            store: me.store,
            columns: me.columns,
            listeners: {
                selectionchange: me.onSelectionChange
            }
        }];

        me.dockedItems = [{
            xtype: 'toolbar',
            dock: 'top',
            items: [{
                xtype: 'button',
                text: 'Start Backup',
                handler: me.onStartBackup
            }]
        }];

        me.callParent();
    },

    onStartBackup: function() {
        var me = this,
            selectedRecords = me.down('backupjobgrid').getSelection();

        if (selectedRecords.length === 0) {
            Ext.Msg.alert('Error', 'No backup job selected');
            return;
        }

        var backupJob = selectedRecords[0],
            vmId = backupJob.get('vmid'),
            storageId = backupJob.get('storageid');

        // Start the backup job
        Proxmox.Utils.API2.request({
            url: '/api2/json/nodes/' + Proxmox.Utils.getNode() + '/qemu/' + vmId + '/backup',
            method: 'POST',
            params: {
                storage: storageId,
                mode: 'snapshot'
            },
            success: function(response) {
                Ext.Msg.alert('Success', 'Backup job started successfully');
            },
            failure: function(response) {
                Ext.Msg.alert('Error', 'Failed to start backup job');
            }
        });
    },

    onSelectionChange: function(selectionModel, selectedRecords) {
        var me = this;

        if (selectedRecords.length === 0) {
            me.down('toolbar').disable();
        } else {
            me.down('toolbar').enable();
        }
    }
});