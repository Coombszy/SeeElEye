# Title: HTTP redirect to HTTPS
# Description: Checks if a website redirects to https
# Version: v0.0.1
# Arguments: 
#   - URL
##########################################################################################
# Using shared lib, import and validate arguments
list = ["URL"] 
from urllib import response
from lib.args import args
args = args(list)
##########################################################################################
import requests

target_url = args["URL"]

# Pretend to be chrome browser
response = requests.get(target_url, headers={'User-Agent': 'Google Chrome'})
redirect = False
hop_url = None
hop_status = None

if response.history:
    for hop in response.history:
        redirect = True
        hop_url = response.url
        hop_status = hop.status_code

if redirect:
    print(f"{target_url} --{hop_status}--> {hop_url}")
    exit(0)
else:
    exit(1)    
