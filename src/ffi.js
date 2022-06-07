function log(txt) {
    const event = new CustomEvent('wasm-log', { text: txt});
    document.body.dispatchEvent(event);
}