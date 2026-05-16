-- Knowledge graph schema in SurrealDB
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
