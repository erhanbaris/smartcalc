const { app, shell, BrowserWindow, Menu, globalShortcut } = require('electron');
const isMac = process.platform === 'darwin';
app.setName("SmartCalc");

const template = [
    ...(isMac ? [{
        label: app.name,
        submenu: [
            { role: 'about' },
            { type: 'separator' },
            { role: 'hide' },
            { role: 'hideothers' },
            { role: 'unhide' },
            { type: 'separator' },
            { role: 'quit' }
        ]
    }] : []),
    // { role: 'fileMenu' }
    {
        label: 'File',
        submenu: [
            isMac ? { role: 'close' } : { role: 'quit' }
        ]
    },
    {
        label: "Edit",
        submenu: [
            { label: "Undo", accelerator: "CmdOrCtrl+Z", selector: "undo:" },
            { label: "Redo", accelerator: "Shift+CmdOrCtrl+Z", selector: "redo:" },
            { type: "separator" },
            { label: "Cut", accelerator: "CmdOrCtrl+X", selector: "cut:" },
            { label: "Copy", accelerator: "CmdOrCtrl+C", selector: "copy:" },
            { label: "Paste", accelerator: "CmdOrCtrl+V", selector: "paste:" },
            { label: "Select All", accelerator: "CmdOrCtrl+A", selector: "selectAll:" }
        ]},
    {
        role: 'Help',
        submenu: [{
            label: 'Learn More',
            click: async() => {
                await shell.openExternal('https://github.com/erhanbaris/smartcalc-app')
            }
        }]
    }
]

const menu = Menu.buildFromTemplate(template)
Menu.setApplicationMenu(menu)

function createWindow() {
    const win = new BrowserWindow({
        width: 1024,
        height: 768,
        transparent: true,
        show: false,
        frame: false,
        center: true,
        webPreferences: {
            nodeIntegration: true,
            nodeIntegrationInWorker: true
        }
    });

    win.loadFile('index.html');
    win.webContents.openDevTools();
    globalShortcut.register('CommandOrControl+Shift+R', () => {});
    globalShortcut.register('CmdOrCtrl+Shift+R', () => {});
    globalShortcut.register('CommandOrControl+R', () => {});
    globalShortcut.register('CmdOrCtrl+R', () => {});
    globalShortcut.register('Ctrl+R', () => {});
    globalShortcut.register('F5', () => {});
    win.once('ready-to-show', () => {
        win.show()
    });
}

app.whenReady().then(() => {
    createWindow()
})

app.on('window-all-closed', function() {
    if (isMac) app.quit()
})