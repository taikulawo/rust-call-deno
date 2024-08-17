globalThis.rustCallback = function rustCallback(data) {
    console.log("Receive rust message:", data);
    return 1
};
