// L'affichage de la bibliothèque. Ce module ne fait aucune requête.

/**
 * Affiche les livres, chacun dans la section de son statut.
 * Les sections qui restent vides sont masquées.
 */
export function renderLibrary(entries) {
  const template = document.querySelector("#book-template");

  // Associe chaque statut à sa section, à partir de l'attribut data-status.
  const sections = new Map(
    [...document.querySelectorAll("section[data-status]")].map((section) => [
      section.dataset.status,
      section,
    ]),
  );

  for (const section of sections.values()) {
    section.querySelector(".book-list").replaceChildren();
  }

  for (const entry of entries) {
    const section = sections.get(entry.status);
    if (!section) {
      console.warn(`Unknown status: ${entry.status}`);
      continue;
    }

    // Copie du contenu du <template>, puis remplissage avec textContent :
    // les valeurs venant de l'API ne sont jamais interprétées comme du HTML.
    const item = template.content.cloneNode(true);
    item.querySelector(".book-title").textContent = entry.title;
    item.querySelector(".book-authors").textContent = entry.authors.join(", ");

    const year = item.querySelector(".book-year");
    if (entry.published_year === null) {
      year.remove();
    } else {
      year.textContent = entry.published_year;
    }

    section.querySelector(".book-list").append(item);
  }

  for (const section of sections.values()) {
    section.hidden = section.querySelector(".book-list").children.length === 0;
  }
}
