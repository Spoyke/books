Description :
- Site web du projet
- Sujets (rust, self-host, ...)
- et une phrase décrivant le projet

README :
- Pourquoi le projet existe
- Comment le compiler et le lancer
- Où il en est (en cours, stable, ...)
- Facultatif -> des exemples d'utilisations (requêtes et réponses)

Liscence

.gitignore propre



Requêtes :

- Récupérer la librairie d'un utilisateur :

```sql
WITH ranked AS (
    SELECT books.id,
           books.uuid,
           books.title,
           editions.published_year,
           readings.status,
           readings.created_at,
           ROW_NUMBER() OVER (
               PARTITION BY books.id
               ORDER BY readings.created_at DESC, readings.id DESC
           ) AS rn
    FROM readings
    JOIN editions ON editions.id = readings.edition_id
    JOIN books    ON books.id    = editions.book_id
    WHERE readings.user_id = 1
)
SELECT ranked.uuid, ranked.title, ranked.published_year, ranked.status, json_group_array(authors.name ORDER BY book_authors.position) AS authors
FROM ranked
JOIN book_authors ON book_authors.book_id = ranked.id
JOIN authors      ON authors.id           = book_authors.author_id
WHERE rn = 1
GROUP BY ranked.id
ORDER BY ranked.created_at DESC;
```
