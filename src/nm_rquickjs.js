// rquickjs Native Messaging host
// Based on https://github.com/guest271314/NativeMessagingHosts/blob/main/nm_qjs_64.js
// guest271314 9-23-2026
function getMessage() {
  const headerBuffer = std.in.read(4);
  if (!headerBuffer || headerBuffer.byteLength === 0) return null;

  const view = new DataView(headerBuffer);
  const messageLength = view.getUint32(0, true);

  const bodyBuffer = std.in.read(messageLength);
  if (!bodyBuffer || bodyBuffer.byteLength === 0) return null;

  return new Uint8Array(bodyBuffer);
}

function sendMessage(message) {
  const COMMA = 44;
  const OPEN_BRACKET = 91;
  const CLOSE_BRACKET = 93;
  const CHUNK_SIZE = 1024 * 1024; // 1MB

  if (message.length <= CHUNK_SIZE) {
    const output = new Uint8Array(4 + message.length);
    output[0] = (message.length >> 0) & 0xff;
    output[1] = (message.length >> 8) & 0xff;
    output[2] = (message.length >> 16) & 0xff;
    output[3] = (message.length >> 24) & 0xff;
    output.set(message, 4);

    std.out.write(output.buffer, 0, output.length);
    std.out.flush();
    return;
  }

  let index = 0;

  while (index < message.length) {
    let splitIndex;
    let searchStart = index + CHUNK_SIZE - 8;

    if (searchStart >= message.length) {
      splitIndex = message.length;
    } else {
      splitIndex = message.indexOf(COMMA, searchStart);
      if (splitIndex === -1) {
        splitIndex = message.length;
      }
    }

    const rawChunk = message.subarray(index, splitIndex);
    const startByte = rawChunk[0];
    const endByte = rawChunk[rawChunk.length - 1];

    let prepend = null;
    let append = null;

    if (startByte === OPEN_BRACKET && endByte !== CLOSE_BRACKET) {
      append = CLOSE_BRACKET;
    } else if (startByte === COMMA) {
      prepend = OPEN_BRACKET;
      if (endByte !== CLOSE_BRACKET) {
        append = CLOSE_BRACKET;
      }
    }

    let bodyLength = rawChunk.length;
    let sourceOffset = 0;
    if (startByte === COMMA) {
      sourceOffset = 1;
      bodyLength -= 1;
    }

    const hasPrepend = prepend !== null;
    const hasAppend = append !== null;

    const totalLength = 4 + (hasPrepend ? 1 : 0) + bodyLength +
      (hasAppend ? 1 : 0);
    const output = new Uint8Array(totalLength);

    const dataLen = totalLength - 4;
    output[0] = (dataLen >> 0) & 0xff;
    output[1] = (dataLen >> 8) & 0xff;
    output[2] = (dataLen >> 16) & 0xff;
    output[3] = (dataLen >> 24) & 0xff;

    let cursor = 4;
    if (hasPrepend) {
      output[cursor] = prepend;
      cursor++;
    } else if (startByte === COMMA) {
      output[cursor] = OPEN_BRACKET;
      cursor++;
    }

    output.set(rawChunk.subarray(sourceOffset), cursor);
    cursor += bodyLength;

    if (hasAppend) {
      output[cursor] = append;
    }

    std.out.write(output.buffer, 0, output.length);
    std.out.flush();

    index = splitIndex;
  }
}

function main() {
  while (true) {
    try {
      const message = getMessage();
      if (!message) return;
      sendMessage(message);
    } catch (err) {
      const errPayload = new Uint8Array(
        Array.from(`Exception: ${err.message}\n`).map((c) => c.charCodeAt(0)),
      );
      std.stderr.write(errPayload.buffer, 0, errPayload.length);
      std.stderr.flush();
    }
  }
}

try {
  main();
} catch (e) {
  std.exit(0);
}
