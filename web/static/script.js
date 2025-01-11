document.addEventListener("DOMContentLoaded", () => {
    const utxoTable = document.getElementById("utxo-table");
    const outixsList = document.getElementById("outixs-list");
    const nostrList = document.getElementById("nostr-list");
    const bitmapList = document.getElementById("bitmap-list");

    const utxos = [
        { txid: "abcd1234", output: "0.01 BTC", address: "bc1example" },
        { txid: "efgh5678", output: "0.02 BTC", address: "bc1example2" },
    ];

    const outixs = [
        { id: "parcel_1", metadata: "data_point_1" },
        { id: "parcel_2", metadata: "data_point_2" },
    ];

    const nostrEvents = [
        { event_id: "event_1", content: "Hello Nostr!" },
        { event_id: "event_2", content: "Relay info here." },
    ];

    const bitmaps = [
        { bitmap_id: "bmp_1", relay_data: "Relay 1 data" },
        { bitmap_id: "bmp_2", relay_data: "Relay 2 data" },
    ];

    utxos.forEach(utxo => {
        const row = document.createElement("tr");
        row.innerHTML = `<td>${utxo.txid}</td><td>${utxo.output}</td><td>${utxo.address}</td>`;
        utxoTable.appendChild(row);
    });

    outixs.forEach(outix => {
        const li = document.createElement("li");
        li.textContent = `${outix.id}: ${outix.metadata}`;
        outixsList.appendChild(li);
    });

    nostrEvents.forEach(event => {
        const li = document.createElement("li");
        li.textContent = `${event.event_id}: ${event.content}`;
        nostrList.appendChild(li);
    });

    bitmaps.forEach(bitmap => {
        const li = document.createElement("li");
        li.textContent = `${bitmap.bitmap_id}: ${bitmap.relay_data}`;
        bitmapList.appendChild(li);
    });
});

