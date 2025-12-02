import datetime

BIRTHDATE = datetime.date(2008, 9, 14)
TEMPLATE_PATH = 'templates/README.md.tpl'
OUTPUT_PATH = 'README.md'

TEMPLATES = {
    "age": "_{_{age}_}_",
    "days_until": "_{_{days_until}_}_"
}

# Get the current UTC time
utc_now = datetime.datetime.now(datetime.timezone.utc)

# Determine if Eastern Time is in Daylight Saving Time (DST)
# Use the second Sunday in March as the start of DST
# Use the first Sunday in November as the end of DST
year = utc_now.year
dst_start = datetime.datetime(year, 3, 8, 2, tzinfo=datetime.timezone.utc)  # Second Sunday in March
dst_start += datetime.timedelta(days=(6 - dst_start.weekday()))  # Adjust to the second Sunday
dst_end = datetime.datetime(year, 11, 1, 2, tzinfo=datetime.timezone.utc)  # First Sunday in November
dst_end += datetime.timedelta(days=(6 - dst_end.weekday()))  # Adjust to the first Sunday

# Check if we're in DST
is_dst = dst_start <= utc_now < dst_end
offset_hours = -4 if is_dst else -5  # UTC-4 during DST, UTC-5 otherwise

# Apply the Eastern Time offset
eastern_time = utc_now + datetime.timedelta(hours=offset_hours)
now = eastern_time.date()

# Calculate age
age = now.year - BIRTHDATE.year
if (now.month, now.day) < (BIRTHDATE.month, BIRTHDATE.day):
    age -= 1

# Calculate days until next birthday
next_birthday_year = now.year if (now.month, now.day) < (BIRTHDATE.month, BIRTHDATE.day) else now.year + 1
next_birthday = datetime.date(next_birthday_year, BIRTHDATE.month, BIRTHDATE.day)
days_until = (next_birthday - now).days

# Read template and replace placeholders
with open(TEMPLATE_PATH, 'r', encoding='utf-8') as tpl:
    content = tpl.read()
content = content.replace(TEMPLATES["age"], str(age))
content = content.replace(TEMPLATES["days_until"], str(days_until))

# Write the updated content to the output file
with open(OUTPUT_PATH, 'w', encoding='utf-8') as out:
    out.write(content)

print(content)  # Debug