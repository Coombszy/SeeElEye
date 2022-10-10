# Title: sample script
# Description: Sample script to show how to use the templatize
# Version: 99.99.99
# Arguments: 
#   - URL
#   - ANOTHER_PARAMETER
#   - THERE_ARE
#   - NO_LIMITS
# 
############################################################################################
# ^
# The format here must be respected, this is so that metadata can be read by the parent CLI
# ^
#
# v
# This allows you to list the required arguments and validate them consistently.
# this code should be copy pastes in all scripts
# v
# Using shared lib, import and validate arguments
list = ["URL", "ANOTHER_PARAMETER", "THERE_ARE", "NO_LIMITS"] 
from lib.args import args
args = args(list)
############################################################################################
# From this point onwards, write your script to do whatever you want
import requests

target_url = args["URL"]
print(f"Target URL: {target_url}")

page = requests.get(target_url)

print(page)




