import datetime

BIRTHDATE = datetime.date(2008, 9, 14)
TEMPLATE_PATH = 'templates/README.md.tpl'
OUTPUT_PATH = 'README.md'

TEMPLATES = {
    "age": "_{_{age}_}_",
    "days_until": "_{_{days_until}_}_"
}

# Calculate age
now = datetime.date.today()
age = now.year - BIRTHDATE.year
if (now.month, now.day) < (BIRTHDATE.month, BIRTHDATE.day):
    age -= 1

# Calculate days until next birthday
next_birthday_year = now.year if (now.month, now.day) < (BIRTHDATE.month, BIRTHDATE.day) else now.year + 1
next_birthday = datetime.date(next_birthday_year, BIRTHDATE.month, BIRTHDATE.day)
days_until = (next_birthday - now).days

# Read template and replace age
with open(TEMPLATE_PATH, 'r', encoding='utf-8') as tpl:
    content = tpl.read()
content = content.replace(TEMPLATES["age"], str(age))
content = content.replace(TEMPLATES["days_until"], str(days_until))

with open(OUTPUT_PATH, 'w', encoding='utf-8') as out:
    out.write(content)

print(content) # Debug
