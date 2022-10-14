# Title: Favicon MD5 Check
# Description: Checks webpage if the Favicon is a generic pre-supplied one
# Version: v0.0.1
# Arguments: 
#   - HTTPS_URL
##########################################################################################
# Using shared lib, import and validate arguments
list = ["HTTPS_URL"] 
from lib.args import args
args = args(list)
##########################################################################################
import requests

target_url = args["HTTPS_URL"]
print(f"Target URL: {target_url}")

page = requests.get(target_url)

print(page)
