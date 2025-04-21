function base64UrlEncode(bytes) {
    const bin = Array.from(bytes, b => String.fromCharCode(b)).join('');
    return btoa(bin)
        .replace(/\+/g, '-')
        .replace(/\//g, '_')
        .replace(/=+$/, ''); // strip padding
}

function base64Decode(b64) {
    const standardB64 = b64
        .replace(/-/g, '+')
        .replace(/_/g, '/')
        .padEnd(b64.length + (4 - b64.length % 4) % 4, '=');

    const bin = atob(standardB64);
    return Uint8Array.from([...bin].map(c => c.charCodeAt(0)));
}

function deflateAndEncode(str) {
    const inputBytes = new TextEncoder().encode(str);
    const compressed = pako.deflate(inputBytes, { raw: true });
    return base64UrlEncode(compressed);
}

function decodeAndInflate(b64) {
    const compressed = base64Decode(b64);
    const decompressed = pako.inflate(compressed, { raw: true });
    return new TextDecoder().decode(decompressed);
}

function updateCodeAndCanvas(editor, canvasInput) {
    const params = new URLSearchParams(window.location.search);
    const code = params.get("code");
    const canvas = params.get("canvas");

    if (code) {
        try {
            let dsl = decodeAndInflate(code);
            editor.setValue(dsl, -1);
        } catch (e) {
            console.warn("Invalid compressed code in ?code", e);
        }
    }

    if (canvas) {
        try {
            canvasInput.value = decodeAndInflate(canvas);
        } catch (e) {
            console.warn("Invalid compressed buffer in ?canvas", e);
        }
    }
}


// Query param utils
function updateQueryParam(key, value) {
    const url = new URL(window.location);

    if (value === null) {
        url.searchParams.delete(key);
    } else {
        url.searchParams.set(key, value);
    }

    history.replaceState(null, "", url);
}