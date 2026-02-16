# schema db

* All fields are not null by default

table: topics_catalogue  
fields:
- id (pk, uuid)
- name (varchar 50)

---
table: users  
fields:
- id (pk, uuid)
- phone (varchar, 13 max length, is_index)
- address (null or jsonb)
- username (varchar 40)
- created (datetime)
- topics (jsonb)
- last_login (datetime)

---
table: validation_codes (simulate cache)
fields:
- user (fk to user.id)
- code (varchar, length 5, is_index)
- expires_at (timestamp)