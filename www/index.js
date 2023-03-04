import * as wasm from "wasm-terminal";

const terminal = document.getElementById("terminal");
const command = document.getElementById("command");
const promptNode = document.getElementById("prompt");

function writeTerminal(expr, message) {
    const promptCopy = promptNode.children[0].cloneNode(true);
    promptCopy.innerHTML += expr;
    const node = document.createElement("div");
    node.appendChild(promptCopy);

    if (message) {
        const outputNode = document.createElement("pre");
        outputNode.innerHTML = message;
        node.appendChild(outputNode);
    }

    terminal.appendChild(node);
}

function prompt() {
    command.value = "";
    command.style.width = `calc(100% - ${command.offsetLeft + 1}px)`;
    terminal.appendChild(promptNode);
    command.focus();
}

function eval_expr(expr) {
    if (!command) {
        writeTerminal();
    } else {
        writeTerminal(expr, wasm.evaluate_expression(expr));
    }
    prompt();

}

eval_expr("help")
eval_expr("welcome")

document.getElementById("form").addEventListener("submit", function(event) {
    event.preventDefault();
    eval_expr(command.value);
});

// wasm.greet("a");
