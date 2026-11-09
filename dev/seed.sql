PRAGMA foreign_keys = ON;

INSERT INTO users (id, uuid, name)
VALUES (1, '00000000-0000-0000-0000-000000000001', 'Simon');

INSERT INTO books (id, uuid, title)
VALUES (1, '00000000-0000-0000-0000-000000000101', 'Dune'),
(2, '00000000-0000-0000-0000-000000000102', 'Good omens'),
(3, '00000000-0000-0000-0000-000000000103', 'The Color of Magic');

INSERT INTO authors (id, uuid, name)
VALUES (1, '00000000-0000-0000-0000-000000000201', 'Frank Herbert'),
(2, '00000000-0000-0000-0000-000000000202', 'Terry Pratchett'),
(3, '00000000-0000-0000-0000-000000000203', 'Neil Gaiman');

INSERT INTO book_authors (book_id, author_id, position)
VALUES (1, 1, 1), (2, 2, 1), (2, 3, 2), (3, 2, 1);

INSERT INTO editions (id, uuid, book_id, published_year, language)
VALUES (1, '00000000-0000-0000-0000-000000000301', 1, 1965, 'en'),
(2, '00000000-0000-0000-0000-000000000302', 2, 1990, 'en'),
(3, '00000000-0000-0000-0000-000000000303', 3, 1983, 'en');

INSERT INTO readings (uuid, user_id, edition_id, status, started_at, finished_at, rating)
VALUES ('00000000-0000-0000-0000-000000000401', 1, 1, 'completed', '2026-01-05', '2026-02-10', 9);

INSERT INTO readings (uuid, user_id, edition_id, status, started_at)
VALUES ('00000000-0000-0000-0000-000000000402', 1, 1, 'active', '2026-03-05');

INSERT INTO readings (uuid, user_id, edition_id, status, started_at, finished_at, rating)
VALUES ('00000000-0000-0000-0000-000000000403', 1, 2, 'completed', '2026-02-05', '2026-03-09', 5);

INSERT INTO readings (uuid, user_id, edition_id, status)
VALUES ('00000000-0000-0000-0000-000000000404', 1, 3, 'pending');
