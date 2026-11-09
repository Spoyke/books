// Point d'entrée : charge la bibliothèque au démarrage de la page.

import { getLibrary } from "./api.js";
import { renderLibrary } from "./library.js";

const message = document.querySelector("#message");

function showMessage(text, { isError = false } = {}) {
  message.textContent = text;
  message.classList.toggle("error", isError);
  message.hidden = false;
}

try {
  const entries = await getLibrary();
  if (entries.length === 0) {
    showMessage("Ta bibliothèque est vide pour l'instant.");
  } else {
    renderLibrary(entries);
  }
} catch (error) {
  console.error(error);
  showMessage("Impossible de charger la bibliothèque. Réessaie plus tard.", {
    isError: true,
  });
}
