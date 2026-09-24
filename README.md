## rquickjs Native Messaging host

> [rquickjs](https://github.com/DelSkayn/rquickjs)
>
> This library is a high level bindings of the [QuickJS-NG](https://quickjs-ng.github.io/quickjs/) JavaScript engine, a fork of the [QuickJS](https://bellard.org/quickjs/) Javascript engine.
> Its goal is to be an easy to use, and safe wrapper similar to the rlua library.

### Compile 
#### Native executable
```shell
cargo build --release
```

#### wasm32-wasip1
```shell
cargo build --release --target wasm32-wasip1
```
### Installation and usage on Chrome and Chromium

1. Navigate to `chrome://extensions`.
2. Toggle `Developer mode`.
3. Click `Load unpacked`.
4. Select `native-messaging-rquickjs` folder.
5. Note the generated extension ID.
6. Open `nm_rquickjs.json` in a text editor, set `"path"` to absolute path of `nm_rquicks` (native executable), or `nm_rquickjs.sh` (shellscript to execute `wasmtime nm_rquickjs.wasm`) and `chrome-extension://<ID>/` using ID from 5 in `"allowed_origins"` array; and make sure `wasmtime` is in `PATH` and `nm_rquickjs.sh` is executable (when executing `nm_rquickjs.wasm` with a WASM runtime).
7. Copy the `nm_rquickjs.json` file to Chrome or Chromium configuration folder, e.g., Chromium on Linux `~/.config/chromium/NativeMessagingHosts`; Chrome dev channel on Linux `~/.config/google-chrome-unstable/NativeMessagingHosts`.
8. To test click `service worker` link in panel of unpacked extension which is DevTools for `background.js` in MV3 `ServiceWorker`, observe echo'ed message from `rquickjs` Native Messaging host. To disconnect run `port.disconnect()`.

The Native Messaging host echoes back the message passed. 

For differences between OS and browser implementations see [Chrome incompatibilities](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Chrome_incompatibilities#native_messaging).

## License
Do What the Fuck You Want to Public License [WTFPLv2](http://www.wtfpl.net/about/)
