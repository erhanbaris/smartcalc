const { app, BrowserWindow, globalShortcut } = require('electron');
const path = require('path');

function createWindow() {
    const win = new BrowserWindow({
        width: 1024,
        height: 768,

        webPreferences: {

        }
    })

    win.loadFile('index.html');

    //win.webContents.openDevTools();

    globalShortcut.register('CommandOrControl+Shift+R', () => {});
    globalShortcut.register('CmdOrCtrl+Shift+R', () => {});
    globalShortcut.register('CommandOrControl+R', () => {});
    globalShortcut.register('CmdOrCtrl+R', () => {});
    globalShortcut.register('Ctrl+R', () => {});
    globalShortcut.register('F5', () => {});
}

app.whenReady().then(() => {
    createWindow();
})

app.on('window-all-closed', function() {
    if (process.platform !== 'darwin') app.quit()
})