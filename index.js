// index.js
import init, { init_rng, reverse, random_numbers } from "./pkg/math_quiz.js";

// On attend que le WASM soit chargé
async function run() {
    await init();
    init_rng(BigInt(Date.now()));

    const outputDiv = document.getElementById("output");
    const inputText = document.getElementById("input");
    const clickButton = document.getElementById("clickMe");

    // Fonction appelée quand on clique
    clickButton.addEventListener("click", () => {
        const reversed = reverse(inputText.value);

        const seed = BigInt(Date.now());
        const random = random_numbers(5);

        outputDiv.innerText = `Texte inversé : ${reversed}\nNombre aléatoire : ${random}`;
    });
}

// Lancer l'init WASM
run();

