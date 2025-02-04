Ext.define('PBS.form.CloudKeySelector', {
    extend: 'Proxmox.form.ComboGrid',
    alias: 'widget.pbsCloudKeySelector',

    allowBlank: false,
    displayField: 'hint',
    valueField: 'fingerprint',
    value: null,
    multiSelect: false,


    store: {
	proxy: {
	    type: 'proxmox',
	    url: '/api2/json/config/cloud-encryption-keys',
	},
	autoLoad: true,
	sorter: 'hint',
    },

    listConfig: {
	columns: [
	    {
		text: gettext('Hint'),
		dataIndex: 'hint',
		sortable: true,
		flex: 1,
		renderer: Ext.String.htmlEncode,
	    },
	    {
		text: gettext('Fingerprint'),
		sortable: true,
		dataIndex: 'fingerprint',
		flex: 5,
	    },
	],
    },
});
