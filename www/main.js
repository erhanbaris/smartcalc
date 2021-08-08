const { app, BrowserWindow, Menu, globalShortcut } = require('electron');
const isMac = process.platform === 'darwin';


const template = [
    ...(isMac ? [{
        label: app.name,
        submenu: [
            { role: 'about' },
            { type: 'separator' },
            { role: 'services' },
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
        role: 'help',
        submenu: [{
            label: 'Learn More',
            click: async() => {
                const { shell } = require('electron')
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
        center: true,
        webPreferences: {
            nodeIntegration: true,
            nodeIntegrationInWorker: true
        }
    })

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
    if (process.platform !== 'darwin') app.quit()
})