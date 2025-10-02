sqlite3 db.sqlite -init sqlite-schema-diagram/sqlite-schema-diagram.sql "" > schema.dot
dot -Tpdf schema.dot > schema.pdf


