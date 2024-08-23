import { op_redirect } from "ext:core/ops";

async function redirect(s: string) {
    await op_redirect(s);
}

globalThis.rustCallback = function rustCallback(data) {
    console.log("Receive rust message:", data);
    return 1;
};
export { redirect };
