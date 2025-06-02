import requests
import gzip
import io
# import pandas as pd # Removed as requested
import urllib.parse # Import for URL decoding (kept for general utility, not strictly needed for params)

# Define the parameters for the query
target_knot_type = "3_1"
target_organism = "Escherichia coli"
target_category = "AF4" # New parameter for Category filter

# Updated base URL to the working browse endpoint
base_url = "https://alphaknot.cent.uw.edu.pl/browse/"

# Define the result columns as per the working URL, URL-decoded
result_columns = "Knot_type;Category;Uniprot;Organism;pLDDT_knotcore;Protein_name;PDB;"

# Construct the query parameters
params = {
    "field": ["Knot_type", "Organism", "Category"], # Added 'Category' field
    "val": [target_knot_type, target_organism, target_category], # Added target_category value
    "conj": ["AND", "AND"],  # Logical AND between the field-value pairs, adjusted for multiple fields
    "raw": "2",      # Changed from '3' to '2' as per working URL
    "result_cols": result_columns # Added result_cols parameter
}

print(f"Attempting to query AlphaKnot API with parameters:")
print(f"  Base URL: {base_url}")
print(f"  Knot Type: {target_knot_type}")
print(f"  Organism: {target_organism}")
print(f"  Category: {target_category}") # Print the new parameter
print(f"  Result Columns: {result_columns}")
print(f"  Raw format: {params['raw']}")

# Make the GET request to the AlphaKnot API
response = requests.get(base_url, params=params, stream=True)

# Check the content type header first
content_type = response.headers.get('content-type', '').lower()

# Variable to hold the raw TSV content string
tsv_content_string = ""

if 'text/html' in content_type:
    print(f"Error: AlphaKnot API returned HTML content instead of expected TSV data.")
    print(f"This might indicate no data found for the query, an API error, or a redirect.")
    print(f"Response content snippet:\n{response.text[:500]}...") # Print first 500 chars of HTML
elif 'content-encoding' in response.headers and 'gzip' in response.headers['content-encoding']:
    # Decompress the gzipped content (less likely for raw=2, but kept for robustness)
    decompressed_content = gzip.decompress(response.content)
    tsv_content_string = decompressed_content.decode('utf-8')
    print(f"Response content was gzipped and successfully decompressed.")
else:
    # If not HTML and not gzipped, assume it's plain TSV (expected for raw=2)
    print("Response content is not gzipped. Assuming plain TSV.")
    tsv_content_string = response.text

if tsv_content_string:
    # Process the TSV content manually
    lines = tsv_content_string.strip().split('\n')
    
    if not lines or (len(lines) == 1 and not lines[0].strip()):
        print("No data received from the API for the given parameters (or empty TSV).")
    else:
        # Print header
        header = lines[0]
        print("\n--- Retrieved Knotted Proteins (Header) ---")
        print(header)

        # Print first few data rows (e.g., 5 rows)
        print("\n--- Retrieved Knotted Proteins (First 5 Data Rows) ---")
        data_rows = lines[1:]
        for i, row in enumerate(data_rows):
            if i >= 5:
                break
            print(row)
        
        print(f"\nTotal entries: {len(data_rows)}") # Total data rows (excluding header)
