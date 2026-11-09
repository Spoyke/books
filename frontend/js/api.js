// Toutes les requêtes vers le serveur. Ce module ne touche jamais au DOM.

/**
 * Récupère la bibliothèque de l'utilisateur.
 * @returns {Promise<Array<{uuid: string, title: string, authors: string[],
 *   published_year: number | null, status: string}>>}
 */
export async function getLibrary() {
  const response = await fetch("/api/library");
  if (!response.ok) {
    throw new Error(`GET /api/library failed with status ${response.status}`);
  }
  return response.json();
}
