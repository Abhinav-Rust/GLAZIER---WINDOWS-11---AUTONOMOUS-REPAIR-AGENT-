-- Knowledge graph schema in SurrealDB
-- Strictly enforcing Vaiśeṣika Padārthas

-- 1. Dravya (Substance)
DEFINE TABLE dravya SCHEMAFULL;
DEFINE FIELD id ON dravya TYPE string;
DEFINE FIELD type ON dravya TYPE string ASSERT $value IN ['HardwareDevice', 'SoftwareComponent', 'SystemResource'];
DEFINE FIELD name ON dravya TYPE string;

-- 2. Guna (Quality) - inheres in Dravya
DEFINE TABLE guna SCHEMAFULL;
DEFINE FIELD id ON guna TYPE string;
DEFINE FIELD type ON guna TYPE string ASSERT $value IN ['State', 'ErrorCode', 'Version'];
DEFINE FIELD value ON guna TYPE string;
DEFINE FIELD dravya_id ON guna TYPE record<dravya>;

-- 3. Karma (Action)
DEFINE TABLE karma SCHEMAFULL;
DEFINE FIELD id ON karma TYPE string;
DEFINE FIELD name ON karma TYPE string;
DEFINE FIELD action_type ON karma TYPE string;

-- 4. Samanya (Universal)
DEFINE TABLE samanya SCHEMAFULL;
DEFINE FIELD id ON samanya TYPE string;
DEFINE FIELD category ON samanya TYPE string;

-- 5. Visesa (Particular)
DEFINE TABLE visesa SCHEMAFULL;
DEFINE FIELD id ON visesa TYPE string;
DEFINE FIELD instance_uuid ON visesa TYPE string;

-- 6. Samavaya (Inherence relation)
DEFINE TABLE samavaya SCHEMAFULL;
DEFINE FIELD in ON samavaya TYPE record<dravya>;
DEFINE FIELD out ON samavaya TYPE record<dravya>;

-- 7. Abhava (Absence)
DEFINE TABLE abhava SCHEMAFULL;
DEFINE FIELD id ON abhava TYPE string;
DEFINE FIELD type ON abhava TYPE string ASSERT $value IN ['Pragabhava', 'Dhvamsabhava', 'Atyantabhava', 'Anyonyabhava'];
DEFINE FIELD target ON abhava TYPE string;


-- Episodic Cases for Upamana
DEFINE TABLE case SCHEMAFULL;
DEFINE FIELD id ON case TYPE string;
DEFINE FIELD timestamp ON case TYPE datetime;
DEFINE FIELD target ON case TYPE string;
DEFINE FIELD symptoms ON case TYPE array;
DEFINE FIELD hypothesis ON case TYPE string;
DEFINE FIELD actions ON case TYPE array;
DEFINE FIELD outcome ON case TYPE string;
DEFINE FIELD guna_start ON case TYPE string;
DEFINE FIELD guna_end ON case TYPE string;
