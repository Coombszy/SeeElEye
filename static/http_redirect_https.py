# Title: HTTP redirect to HTTPS
# Description: Checks if a website redirects to https
# Version: v0.0.1
# Arguments: 
#   - HTTP_URL
##########################################################################################
# Using shared lib, import and validate arguments
list = ["HTTP_URL"] 
from lib.args import args
args = args(list)
##########################################################################################
from urllib import response
import requests

target_url = args["HTTP_URL"]

# Pretend to be chrome browser
response = requests.head(target_url, headers={'User-Agent': 'Google Chrome'}, allow_redirects=True)
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
