-- Seeding script
INSERT INTO nip05 (pubkey, name)
VALUES (
        '5be6446aa8a31c11b3b453bf8dafc9b346ff328d1fa11a0fa02a1e6461f6a9b1',
        'tony'
    ),
    (
        '5be6446aa8a31c11b3b453bf8dafc9b346ff328d1fa11a0fa02a1e6461f6a9b1',
        'testy'
    ),
    (
        'de8823cdc979bf4c753223edc19a7abc35ecff2959ef50ca9e9d573ac0f83fd0',
        'kody'
    ),
    (
        '5be6446aa8a31c11b3b453bf8dafc9b346ff328d1fa11a0fa02a1e6461f6a9b1',
        'test'
    ),
    (
        'e1ff3bfdd4e40315959b08b4fcc8245eaa514637e1d4ec2ae166b743341be1af',
        'carman'
    );
INSERT INTO relay (relay)
VALUES ('wss://nostr.mutinywallet.com');
-- Get the IDs of the inserted rows
DO $$
DECLARE relay_id INTEGER;
tony_id INTEGER;
testy_id INTEGER;
kody_id INTEGER;
test_id INTEGER;
carman_id INTEGER;
BEGIN
SELECT id INTO relay_id
FROM relay
WHERE relay = 'wss://nostr.mutinywallet.com';
SELECT id INTO tony_id
FROM nip05
WHERE name = 'tony';
SELECT id INTO testy_id
FROM nip05
WHERE name = 'testy';
SELECT id INTO kody_id
FROM nip05
WHERE name = 'kody';
SELECT id INTO test_id
FROM nip05
WHERE name = 'test';
SELECT id INTO carman_id
FROM nip05
WHERE name = 'carman';
-- Insert into nip05relays
INSERT INTO nip05relays (nip05_id, relay_id)
VALUES (tony_id, relay_id),
    (testy_id, relay_id),
    (kody_id, relay_id),
    (test_id, relay_id),
    (carman_id, relay_id);
END $$;
