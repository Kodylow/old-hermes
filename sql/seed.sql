-- Seeding script
INSERT INTO user (pubkey, name, dm_type)
VALUES (
        '5be6446aa8a31c11b3b453bf8dafc9b346ff328d1fa11a0fa02a1e6461f6a9b1',
        'tony',
        'nostr'
    ),
    (
        '5be6446aa8a31c11b3b453bf8dafc9b346ff328d1fa11a0fa02a1e6461f6a9b1',
        'testy',
        'nostr'
    ),
    (
        'de8823cdc979bf4c753223edc19a7abc35ecff2959ef50ca9e9d573ac0f83fd0',
        'kody',
        'nostr'
    ),
    (
        '5be6446aa8a31c11b3b453bf8dafc9b346ff328d1fa11a0fa02a1e6461f6a9b1',
        'test',
        'nostr'
    ),
    (
        'e1ff3bfdd4e40315959b08b4fcc8245eaa514637e1d4ec2ae166b743341be1af',
        'carman',
        'nostr'
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
FROM user
WHERE name = 'tony';
SELECT id INTO testy_id
FROM user
WHERE name = 'testy';
SELECT id INTO kody_id
FROM user
WHERE name = 'kody';
SELECT id INTO test_id
FROM user
WHERE name = 'test';
SELECT id INTO carman_id
FROM user
WHERE name = 'carman';
-- Insert into userrelays
INSERT INTO userrelays (user_id, relay_id)
VALUES (tony_id, relay_id),
    (testy_id, relay_id),
    (kody_id, relay_id),
    (test_id, relay_id),
    (carman_id, relay_id);
END $$;
