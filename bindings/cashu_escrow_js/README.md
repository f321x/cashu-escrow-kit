# Cashu Escrow Kit Bindings

Javascript and TS bindings for cashu-escrow-kit.

## Building the wasm module
In the bindings/cashu_escrow_js directory, run:
```sh
wasm-pack build --dev
```

## Run the web client
In the web_client directory, run:
```sh
npm install
npm run build
npm run start
```

Then open the browser and go to http://localhost:8081/. To test the escrow workflow, start a coordinator process, the mint and the local nostr relay. Then you can open two tabs and select different roles, buyer and seller. Press then the "Start" button after filling in the required fields.