# Title: Favicon MD5 Check
# Description: Checks webpage if the Favicon is a generic pre-supplied one
# Version: v0.0.1
# Arguments: 
#   - URL
##########################################################################################
# Using shared lib, import and validate arguments
list = ["URL"]
from lib.args import args
args = args(list)
##########################################################################################
import requests

target_url = args["URL"]

if "http" in target_url:
    print("URL must not contain http or https")
    exit(1)

target_url = "https://"+target_url

print(f"Target URL: {target_url}")

page = requests.get(target_url)

print(page)
