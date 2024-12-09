Path ← "uuids.txt"

Database ← &fras Path

┌─╴UUID
  # Counts the number of UUIDS in the database
  NumberOfUUID ← ÷36 ⧻
  # give the index of the uuid in the database
  WhereIs ← ⊚≡/+⌕
  # Add UUID to datbase
  Append ← ↯⇌[36 NumberOfUUID] . ⊂
  # Remove 1 instance of a UUID from the database
  Remove = ▽≠1⊸⦷
└─╴

UUID~Append Database
UUID~WhereIs "c8cf24cc-7e49-4780-8e52-bb891295d92e" .
?
&s
&s.

&fwa Path
