import pandas as pd
import sys
import re

# Read CSV
df = pd.read_csv("cert-matrix/cert-matrix.csv")

# Convert to Markdown table
def to_md_table(df):
    header = "| " + " | ".join(df.columns) + " |"
    sep = "|" + "---|" * len(df.columns)
    rows = ["| " + " | ".join(str(cell) for cell in row) + " |" for row in df.values]
    return "\n".join([header, sep] + rows)

md_table = to_md_table(df)

# Read README
with open("README.md", encoding="utf-8") as f:
    readme = f.read()

# Replace the Certification Matrix section
def replace_matrix_section(text, new_table):
    pattern = r'(## Certification Matrix\n.*?\n)(?:_Table to be inserted or linked to here\._|\| .+?\|\n\|---.+?\|\n(?:\|.+?\|\n)+)'  # matches old table or placeholder
    replacement = r'\1' + new_table + '\n'
    new_text, n = re.subn(pattern, replacement, text, flags=re.DOTALL)
    if n == 0:
        # fallback: insert after section header
        return re.sub(r'(## Certification Matrix\n.*?\n)', r'\1' + new_table + '\n', text, flags=re.DOTALL)
    return new_text

new_readme = replace_matrix_section(readme, md_table)

with open("README.md", "w", encoding="utf-8") as f:
    f.write(new_readme)
